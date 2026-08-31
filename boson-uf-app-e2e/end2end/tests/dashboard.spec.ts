import { test, expect, seedAuth, waitForHydrated } from "./fixtures";

test.describe("pw-boson-dashboard", () => {
  test("pw-boson-dashboard-happy-kpis", async ({ page }) => {
    const seeded = await seedAuth(page, "admin");
    await page.goto("/boson", { waitUntil: "domcontentloaded" });
    await waitForHydrated(page);
    await expect(page.getByTestId("boson-dashboard")).toBeVisible({ timeout: 60_000 });
    await expect(page.getByTestId("dashboard-stat-tasks")).toBeVisible();
    // Seeded registry task must appear in recent tasks / KPI surface.
    await expect(page.getByText(seeded.fixtures.task_name).first()).toBeVisible({
      timeout: 60_000,
    });
  });

  test("pw-boson-dashboard-sad-empty-recent-not-crash", async ({ page }) => {
    await seedAuth(page, "admin");
    await page.goto("/boson", { waitUntil: "domcontentloaded" });
    await waitForHydrated(page);
    await expect(page.getByTestId("boson-dashboard")).toBeVisible({ timeout: 60_000 });
    // Dashboard remains mounted even when trend/series windows are sparse.
    await expect(page.getByTestId("dashboard-stat-queued")).toBeVisible();
    await expect(page.getByTestId("orbital-boot-overlay")).toHaveCount(0);
  });
});
