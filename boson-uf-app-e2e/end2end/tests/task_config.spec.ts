import { test, expect, seedAuth, waitForHydrated } from "./fixtures";

test.describe("pw-boson-task-config", () => {
  test("pw-boson-task-config-sad-unverified-email", async ({ page }) => {
    const seeded = await seedAuth(page, "unverified");
    await page.goto(
      `/boson/tasks/${encodeURIComponent(seeded.fixtures.task_name)}/config`,
      { waitUntil: "domcontentloaded" },
    );
    await waitForHydrated(page);
    await expect(page.getByTestId("boson-task-config")).toHaveCount(0);
    await expect(
      page.getByTestId("email-verification-required-empty-state"),
    ).toBeAttached({ timeout: 60_000 });
  });

  test("pw-boson-task-config-happy-admin-save", async ({ page }) => {
    const seeded = await seedAuth(page, "admin");
    await page.goto(
      `/boson/tasks/${encodeURIComponent(seeded.fixtures.task_name)}/config`,
      { waitUntil: "domcontentloaded" },
    );
    await waitForHydrated(page);
    await expect(page.getByTestId("boson-task-config")).toBeVisible({ timeout: 60_000 });
    await expect(page.getByTestId("task-config-save")).toBeVisible({ timeout: 60_000 });
    await page.getByTestId("task-config-save").locator("button").click();
    // Save succeeds without an error MessageBar (navigate or stay on form).
    await expect(page.locator(".orbital-message-bar--error")).toHaveCount(0, {
      timeout: 30_000,
    });
  });
});
