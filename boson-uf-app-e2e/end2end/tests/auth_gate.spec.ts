import { test, expect, seedAuth, waitForHydrated } from "./fixtures";

test.describe("pw-boson-auth-gate", () => {
  test("pw-boson-auth-gate-sad-anonymous", async ({ page }) => {
    await seedAuth(page, "anonymous");
    await page.goto("/boson", { waitUntil: "domcontentloaded" });
    await waitForHydrated(page);
    await expect(page.getByTestId("auth-required-empty-state")).toBeAttached({
      timeout: 60_000,
    });
    await expect(page.getByTestId("boson-dashboard")).toHaveCount(0);
  });

  test("pw-boson-auth-gate-happy-admin", async ({ page }) => {
    await seedAuth(page, "admin");
    await page.goto("/boson", { waitUntil: "domcontentloaded" });
    await waitForHydrated(page);
    await expect(page.getByTestId("boson-app-root")).toBeVisible({ timeout: 60_000 });
    await expect(page.getByTestId("boson-dashboard")).toBeVisible({ timeout: 60_000 });
  });
});
