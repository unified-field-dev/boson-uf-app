import { test, expect, seedAuth, waitForHydrated, expectMutationDenied } from "./fixtures";

test.describe("pw-boson-queue-cancel", () => {
  test("pw-boson-queue-cancel-happy-admin", async ({ page }) => {
    const seeded = await seedAuth(page, "admin", { refreshJob: true });
    await page.goto("/boson/queue", { waitUntil: "domcontentloaded" });
    await waitForHydrated(page);
    await expect(page.getByTestId("boson-queue")).toBeVisible({ timeout: 60_000 });
    const cancel = page.getByTestId(`job-cancel-${seeded.fixtures.job_id}`);
    await expect(cancel).toBeVisible({ timeout: 60_000 });
    await cancel.locator("button").click();
    // Cancelled jobs drop out of the default queued/running table or lose cancel affordance.
    await expect(cancel).toHaveCount(0, { timeout: 60_000 });
  });

  test("pw-boson-queue-cancel-sad-non-admin", async ({ page }) => {
    const seeded = await seedAuth(page, "outsider", { refreshJob: true });
    await page.goto("/boson/queue", { waitUntil: "domcontentloaded" });
    await waitForHydrated(page);
    await expect(page.getByTestId("boson-queue")).toBeVisible({ timeout: 60_000 });
    const cancel = page.getByTestId(`job-cancel-${seeded.fixtures.job_id}`);
    await expect(cancel).toBeVisible({ timeout: 60_000 });
    await cancel.locator("button").click();
    await expectMutationDenied(page);
    // Denied cancel must leave the job cancel control in place.
    await expect(cancel).toBeVisible();
  });
});
