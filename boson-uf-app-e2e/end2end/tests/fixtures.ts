import { test as base, expect, type Page } from "@playwright/test";

export type SeedAuthKind = "anonymous" | "admin" | "outsider" | "unverified";

export type SeedFixtures = {
  task_name: string;
  job_id: string;
  run_id: string;
};

export async function seedAuth(
  page: Page,
  auth: SeedAuthKind,
  opts?: { refreshJob?: boolean },
) {
  const res = await page.request.post("/api/test/seed-data", {
    data: {
      auth,
      refresh_job: opts?.refreshJob ?? false,
    },
  });
  expect(res.ok()).toBeTruthy();
  return res.json() as Promise<{
    ok: boolean;
    auth: string;
    fixtures: SeedFixtures;
  }>;
}

async function bootState(page: Page): Promise<"ready" | "error" | "loading"> {
  return page.evaluate(() => {
    const html = document.documentElement;
    if (html.getAttribute("data-orbital-hydrated") === "true") {
      return "ready";
    }
    if (html.getAttribute("data-orbital-boot-state") === "error") {
      return "error";
    }
    return "loading";
  });
}

/** Cache-busting full navigation — plain reload can reuse a bad wasm fetch. */
async function hardRefresh(page: Page) {
  const url = new URL(page.url());
  url.searchParams.set("_orbboot", String(Date.now()));
  await page.goto(url.toString(), { waitUntil: "load" });
}

/**
 * Wait for Orbital hydrate to mark the document ready, then clear the boot overlay.
 *
 * Large WASM (~100MiB) can fail the first fetch on CI. On `error`, hard-refresh
 * immediately (do not burn the poll budget). Never refresh while still `loading`
 * — that aborts in-flight `.wasm` and sticks boot-state on error.
 */
export async function waitForHydrated(page: Page, timeoutMs = 120_000) {
  const deadline = Date.now() + timeoutMs;
  let refreshes = 0;
  const maxRefreshes = 6;
  while (Date.now() < deadline) {
    const state = await bootState(page);
    if (state === "ready") {
      break;
    }
    if (state === "error") {
      if (refreshes >= maxRefreshes) {
        break;
      }
      refreshes += 1;
      await hardRefresh(page);
      continue;
    }
    await page.waitForTimeout(250);
  }
  await expect
    .poll(async () => bootState(page), { timeout: 5_000 })
    .toBe("ready");
  await expect(page.getByTestId("orbital-boot-overlay")).toHaveCount(0, {
    timeout: 60_000,
  });
  await expect(page.getByTestId("e2e-auth-bootstrap")).toBeAttached({
    timeout: 30_000,
  });
}

/** Higgs / server-fn deny surfaces as an Orbital error MessageBar. */
export async function expectMutationDenied(page: Page) {
  await expect(page.locator(".orbital-message-bar--error").first()).toBeVisible({
    timeout: 60_000,
  });
}

export const test = base;
export { expect };
