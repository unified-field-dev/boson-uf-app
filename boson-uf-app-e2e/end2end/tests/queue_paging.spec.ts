import { test, expect, seedAuth, waitForHydrated } from "./fixtures";
import type { Locator, Page, Request } from "@playwright/test";

// The seed adds 90 jobs `e2e-page-000` ... `e2e-page-089`, alternating queued and failed,
// dated after every other job. With the queued filter the table spans at least three
// 20-row pages; page 1 may start with queued jobs other specs enqueued.

const VARIANTS = [
  { name: "status-filter-desktop-light", scheme: "light", viewport: { width: 1280, height: 800 }, filter: true },
  { name: "status-filter-desktop-dark", scheme: "dark", viewport: { width: 1280, height: 800 }, filter: true },
  // The filter panel's popover opens partly off-screen at phone width, so the
  // phone run pages the unfiltered list (also a cursor path).
  { name: "unfiltered-phone", scheme: "light", viewport: { width: 390, height: 844 }, filter: false },
] as const;

async function selectOption(wrapper: Locator, value: string) {
  const select = wrapper.locator("select");
  await expect(async () => {
    await select.selectOption(value);
    await expect(select).toHaveValue(value);
  }).toPass({ timeout: 5_000 });
}

async function filterQueued(page: Page, table: Locator) {
  await table.getByTestId("data-table-filter-panel-trigger").click();
  const panel = page.getByTestId("data-table-filter-panel");
  await selectOption(panel.getByTestId("data-table-filter-rule-0-field"), "status");
  await selectOption(panel.getByTestId("data-table-filter-rule-0-operator"), "is");
  await panel.getByTestId("data-table-filter-rule-0-value").locator("input").fill("queued");
  const filtered = page.waitForResponse((resp) => isJobsPageRequest(resp.request()));
  await panel.getByRole("button", { name: "Apply" }).click();
  await filtered;
}

/** Job ids on the current page, read from the job id cells. */
async function pageJobIds(table: Locator): Promise<string[]> {
  const ids = await table
    .locator('[data-testid^="job-card-"]')
    .evaluateAll((els) =>
      els.map((el) => (el.getAttribute("data-testid") ?? "").replace(/^job-card-/, "")),
    );
  return ids;
}

/** True when an id is one of the failed (odd) paging seed jobs. */
function isFailedSeedId(id: string) {
  const m = /^e2e-page-(\d{3})$/.exec(id);
  return m !== null && Number(m[1]) % 2 === 1;
}

/** With the status filter on, paging seed ids must be the queued (even) ones. */
function expectOnlyQueuedSeedIds(ids: string[], filtered: boolean) {
  if (!filtered) return;
  expect(ids.filter(isFailedSeedId), "failed seed jobs on a queued page").toEqual([]);
}

function isJobsPageRequest(req: Request) {
  return req.method() === "POST" && req.url().includes("list_jobs_datatable_page");
}

/** True when the server-fn body carries a non-empty cursor token. */
function carriesCursor(req: Request) {
  const body = decodeURIComponent(req.postData() ?? "");
  return /cursor\W{1,4}[A-Za-z0-9_-]{8,}/.test(body);
}

for (const variant of VARIANTS) {
  test.describe("pw-boson-queue-paging", () => {
    test.use({ viewport: variant.viewport });

    test(`pw-boson-queue-paging-happy-${variant.name}`, async ({ page }) => {
      await page.emulateMedia({ colorScheme: variant.scheme });
      await seedAuth(page, "admin", { pagingJobs: true });
      await page.goto("/boson/queue", { waitUntil: "domcontentloaded" });
      await waitForHydrated(page);
      const table = page.getByTestId("boson-queue-data-table");
      await expect(table).toBeVisible({ timeout: 60_000 });
      const footer = table.getByTestId("data-table-footer");
      const bar = table.getByTestId("data-table-pagination");
      const next = bar.getByRole("button", { name: "Next page" });
      const previous = bar.getByRole("button", { name: "Previous page" });

      const filtered = variant.filter;
      if (filtered) await filterQueued(page, table);
      await expect(footer).toContainText("1–20", { timeout: 60_000 });
      await expect
        .poll(async () => {
          const ids = await pageJobIds(table);
          return ids.length === 20 && !(filtered && ids.some(isFailedSeedId));
        })
        .toBe(true);
      const page1 = await pageJobIds(table);
      expectOnlyQueuedSeedIds(page1, filtered);

      // Next continues from the cursor page 1 returned.
      const nextReq = page.waitForRequest(isJobsPageRequest);
      await next.click();
      expect(carriesCursor(await nextReq), "Next sends the page 1 cursor").toBe(true);
      await expect(footer).toContainText("21–40", { timeout: 60_000 });
      await expect.poll(async () => (await pageJobIds(table))[0]).not.toBe(page1[0]);
      const page2 = await pageJobIds(table);
      expect(page2).toHaveLength(20);
      expectOnlyQueuedSeedIds(page2, filtered);
      expect(page2.filter((id) => page1.includes(id))).toEqual([]);

      await next.click();
      await expect(footer).toContainText("41–", { timeout: 60_000 });
      await expect.poll(async () => page2.includes((await pageJobIds(table))[0] ?? "")).toBe(false);
      const page3 = await pageJobIds(table);
      expect(page3.length).toBeGreaterThan(0);
      expectOnlyQueuedSeedIds(page3, filtered);
      expect(page3.filter((id) => page1.includes(id) || page2.includes(id))).toEqual([]);
      // The last queued seed job is on the final page.
      if (filtered && !(await next.isEnabled())) {
        expect(page3).toContain("e2e-page-088");
      }

      // Previous replays the recorded cursor and shows page 2 again, then page 1.
      await previous.click();
      await expect(footer).toContainText("21–40", { timeout: 60_000 });
      await expect.poll(async () => pageJobIds(table)).toEqual(page2);
      await previous.click();
      await expect(footer).toContainText("1–20", { timeout: 60_000 });
      await expect.poll(async () => pageJobIds(table)).toEqual(page1);
    });
  });
}
