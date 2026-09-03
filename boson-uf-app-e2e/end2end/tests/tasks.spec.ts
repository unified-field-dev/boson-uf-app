import { test, expect, seedAuth, waitForHydrated } from "./fixtures";

test.describe("pw-boson-tasks", () => {
  test("pw-boson-tasks-happy-list-detail", async ({ page }) => {
    const seeded = await seedAuth(page, "admin");
    await page.goto("/boson/tasks", { waitUntil: "domcontentloaded" });
    await waitForHydrated(page);
    await expect(page.getByTestId("boson-tasks")).toBeVisible({ timeout: 60_000 });
    await expect(page.getByTestId("boson-tasks-data-table")).toBeVisible({ timeout: 60_000 });
    await expect(page.getByText(seeded.fixtures.task_name).first()).toBeVisible({
      timeout: 60_000,
    });
    await page.goto(`/boson/tasks/${encodeURIComponent(seeded.fixtures.task_name)}`, {
      waitUntil: "domcontentloaded",
    });
    await waitForHydrated(page);
    await expect(page.getByTestId("boson-task-detail")).toBeVisible({ timeout: 60_000 });
    await expect(page.getByText(seeded.fixtures.task_name).first()).toBeVisible();
  });

  test("pw-boson-tasks-sad-unknown-task", async ({ page }) => {
    await seedAuth(page, "admin");
    await page.goto("/boson/tasks/__boson_e2e_no_such_task__", {
      waitUntil: "domcontentloaded",
    });
    await waitForHydrated(page);
    await expect(page.getByTestId("boson-task-detail")).toBeVisible({ timeout: 60_000 });
    await expect(page.getByText(/not found|Task not found|Missing/i).first()).toBeVisible({
      timeout: 60_000,
    });
  });
});
