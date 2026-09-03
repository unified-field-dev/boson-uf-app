import { test, expect, seedAuth, waitForHydrated } from "./fixtures";
import type { Page } from "@playwright/test";

async function completeVisibleTour(page: Page) {
  const footer = page.locator('[data-testid="spotlight-footer"]:visible');
  const next = footer.getByTestId("spotlight-tour-next");
  await expect(footer).toBeVisible({ timeout: 60_000 });
  for (let i = 0; i < 24; i++) {
    if ((await footer.count()) === 0) {
      break;
    }
    // Spotlight panels can sit partially off-screen; DOM click avoids Playwright
    // viewport hit-testing failures that still occur with { force: true }.
    await next.evaluate((el: HTMLElement) => el.click());
    try {
      await expect(footer).toHaveCount(0, { timeout: 2_000 });
      break;
    } catch {
      /* more steps */
    }
  }
  await expect(footer).toHaveCount(0, { timeout: 30_000 });
}

test.describe("help-spotlight", () => {
  test("help-spotlight-skips-when-seeded", async ({ page }) => {
    await seedAuth(page, "admin");
    await page.goto("/boson", { waitUntil: "domcontentloaded" });
    await waitForHydrated(page);
    await expect(page.getByTestId("boson-dashboard")).toBeVisible({ timeout: 60_000 });
    await expect(page.getByTestId("help-step-boson-intro")).toHaveCount(0);
    await expect(page.locator('[data-testid="spotlight-footer"]:visible')).toHaveCount(0);
  });

  test("help-spotlight-skips-auth-gate", async ({ page }) => {
    await seedAuth(page, "anonymous", { help_tour: true });
    await page.goto("/boson", { waitUntil: "domcontentloaded" });
    await waitForHydrated(page);
    await expect(page.getByTestId("auth-required-empty-state")).toBeAttached({
      timeout: 60_000,
    });
    await expect(page.getByTestId("help-step-boson-intro")).toHaveCount(0);
    await expect(page.locator('[data-testid="spotlight-footer"]:visible')).toHaveCount(0);
  });

  test("help-spotlight-dashboard-green", async ({ page }) => {
    await seedAuth(page, "admin", { help_tour: true });
    await page.goto("/boson", { waitUntil: "domcontentloaded" });
    await waitForHydrated(page);
    await expect(page.getByTestId("help-step-boson-intro")).toBeVisible({ timeout: 60_000 });
    await completeVisibleTour(page);
    await expect(page.getByTestId("help-step-boson-intro")).toHaveCount(0);

    await page.reload({ waitUntil: "domcontentloaded" });
    await waitForHydrated(page);
    await expect(page.getByTestId("help-step-boson-intro")).toHaveCount(0);
  });

  test("help-spotlight-tasks-green", async ({ page }) => {
    await seedAuth(page, "admin", { help_tour: true });
    await page.goto("/boson/tasks", { waitUntil: "domcontentloaded" });
    await waitForHydrated(page);
    await expect(page.getByTestId("help-step-boson-tasks-intro")).toBeVisible({
      timeout: 60_000,
    });
    await completeVisibleTour(page);
  });

  test("help-spotlight-task-detail-green", async ({ page }) => {
    const seeded = await seedAuth(page, "admin", { help_tour: true });
    const task = seeded.fixtures.task_name;
    await page.goto(`/boson/tasks/${encodeURIComponent(task)}`, {
      waitUntil: "domcontentloaded",
    });
    await waitForHydrated(page);
    await expect(page.getByTestId("help-step-boson-task-detail-summary")).toBeVisible({
      timeout: 60_000,
    });
    await completeVisibleTour(page);
  });

  test("help-spotlight-task-config-green", async ({ page }) => {
    const seeded = await seedAuth(page, "admin", { help_tour: true });
    const task = seeded.fixtures.task_name;
    await page.goto(`/boson/tasks/${encodeURIComponent(task)}/config`, {
      waitUntil: "domcontentloaded",
    });
    await waitForHydrated(page);
    await expect(page.getByTestId("help-step-boson-task-config-intro")).toBeVisible({
      timeout: 60_000,
    });
    await completeVisibleTour(page);
  });

  test("help-spotlight-queue-green", async ({ page }) => {
    await seedAuth(page, "admin", { help_tour: true, refreshJob: true });
    await page.goto("/boson/queue", { waitUntil: "domcontentloaded" });
    await waitForHydrated(page);
    await expect(page.getByTestId("help-step-boson-queue-intro")).toBeVisible({
      timeout: 60_000,
    });
    await completeVisibleTour(page);
  });

  test("help-spotlight-runs-green", async ({ page }) => {
    await seedAuth(page, "admin", { help_tour: true });
    await page.goto("/boson/runs", { waitUntil: "domcontentloaded" });
    await waitForHydrated(page);
    await expect(page.getByTestId("help-step-boson-runs-intro")).toBeVisible({
      timeout: 60_000,
    });
    await completeVisibleTour(page);
  });

  test("help-spotlight-run-detail-green", async ({ page }) => {
    const seeded = await seedAuth(page, "admin", { help_tour: true });
    const runId = seeded.fixtures.run_id;
    await page.goto(`/boson/runs/${encodeURIComponent(runId)}`, {
      waitUntil: "domcontentloaded",
    });
    await waitForHydrated(page);
    await expect(page.getByTestId("help-step-boson-run-detail-info")).toBeVisible({
      timeout: 60_000,
    });
    await completeVisibleTour(page);
  });
});
