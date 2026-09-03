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

/**
 * Wait for Orbital hydrate to mark the document ready, then clear the boot overlay.
 *
 * Reload immediately when boot enters `error` (do not burn the full poll budget).
 * Never reload while still `loading` — that aborts in-flight `.wasm`.
 */
export async function waitForHydrated(page: Page, timeoutMs = 90_000) {
  const deadline = Date.now() + timeoutMs;
  let reloads = 0;
  while (Date.now() < deadline) {
    const state = await bootState(page);
    if (state === "ready") {
      break;
    }
    if (state === "error" && reloads < 2) {
      reloads += 1;
      await page.reload({ waitUntil: "load" });
      continue;
    }
    await page.waitForTimeout(250);
  }
  await expect.poll(async () => bootState(page), { timeout: 5_000 }).toBe("ready");
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
