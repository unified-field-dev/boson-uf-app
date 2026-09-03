import { test, expect, seedAuth, waitForHydrated } from "./fixtures";

test.describe("pw-boson-runs", () => {
  test("pw-boson-runs-happy-list-detail", async ({ page }) => {
    const seeded = await seedAuth(page, "admin");
    await page.goto("/boson/runs", { waitUntil: "domcontentloaded" });
    await waitForHydrated(page);
    await expect(page.getByTestId("boson-runs")).toBeVisible({ timeout: 60_000 });
    await expect(page.getByTestId("boson-runs-data-table")).toBeVisible({ timeout: 60_000 });
    await page.goto(`/boson/runs/${encodeURIComponent(seeded.fixtures.run_id)}`, {
      waitUntil: "domcontentloaded",
    });
    await waitForHydrated(page);
    await expect(page.getByTestId("boson-run-detail")).toBeVisible({ timeout: 60_000 });
    await expect(page.getByText(seeded.fixtures.run_id).first()).toBeVisible();
  });

  test("pw-boson-runs-sad-unknown-run", async ({ page }) => {
    await seedAuth(page, "admin");
    await page.goto("/boson/runs/__boson_e2e_no_such_run__", {
      waitUntil: "domcontentloaded",
    });
    await waitForHydrated(page);
    await expect(page.getByTestId("boson-run-detail")).toBeVisible({ timeout: 60_000 });
    await expect(page.getByText(/Run not found/i)).toBeVisible({ timeout: 60_000 });
  });
});
