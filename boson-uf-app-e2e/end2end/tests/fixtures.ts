import { test as base, expect, type Page } from "@playwright/test";

export type SeedAuthKind = "anonymous" | "admin" | "outsider" | "unverified";

export type SeedFixtures = {
  task_name: string;
  job_id: string;
  run_id: string;
};

/** All Boson Help inventory keys — seed as seen so non-tour specs stay quiet. */
const BOSON_HELP_STEPS_SEEN = [
  { route: "/boson", feature_highlight: "boson-intro", spotlight: null, replay: false },
  {
    route: "/boson",
    feature_highlight: "boson-dashboard-stats",
    spotlight: "boson-dashboard-stats",
    replay: false,
  },
  {
    route: "/boson",
    feature_highlight: "boson-run-trend",
    spotlight: "boson-dashboard-run-trend",
    replay: false,
  },
  {
    route: "/boson",
    feature_highlight: "boson-ql-tasks",
    spotlight: "boson-ql-tasks",
    replay: false,
  },
  {
    route: "/boson",
    feature_highlight: "boson-ql-queue",
    spotlight: "boson-ql-queue",
    replay: false,
  },
  {
    route: "/boson",
    feature_highlight: "boson-ql-runs",
    spotlight: "boson-ql-runs",
    replay: false,
  },
  {
    route: "/boson",
    feature_highlight: "boson-tasks-overview",
    spotlight: "boson-dashboard-tasks-overview",
    replay: false,
  },
  {
    route: "/boson",
    feature_highlight: "boson-nav",
    spotlight: "boson-nav",
    replay: false,
  },
  {
    route: "/boson/tasks",
    feature_highlight: "boson-tasks-intro",
    spotlight: "boson-tasks",
    replay: false,
  },
  {
    route: "/boson/tasks",
    feature_highlight: "boson-tasks-search",
    spotlight: "boson-tasks-search",
    replay: false,
  },
  {
    route: "/boson/tasks",
    feature_highlight: "boson-tasks-table",
    spotlight: "boson-tasks-data-table",
    replay: false,
  },
  {
    route: "/boson/tasks",
    feature_highlight: "boson-tasks-view",
    spotlight: "boson-tasks-action-view",
    replay: false,
  },
  {
    route: "/boson/tasks",
    feature_highlight: "boson-tasks-configure",
    spotlight: "boson-tasks-action-configure",
    replay: false,
  },
  {
    route: "/boson/tasks",
    feature_highlight: "boson-tasks-view-queue",
    spotlight: "boson-tasks-action-queue",
    replay: false,
  },
  {
    route: "/boson/tasks",
    feature_highlight: "boson-tasks-view-runs",
    spotlight: "boson-tasks-action-runs",
    replay: false,
  },
  {
    route: "/boson/tasks/:task_name",
    feature_highlight: "boson-task-detail-summary",
    spotlight: "boson-task-detail-summary",
    replay: false,
  },
  {
    route: "/boson/tasks/:task_name",
    feature_highlight: "boson-task-detail-metrics",
    spotlight: "boson-task-detail-metrics",
    replay: false,
  },
  {
    route: "/boson/tasks/:task_name",
    feature_highlight: "boson-task-detail-configure",
    spotlight: "task-detail-configure",
    replay: false,
  },
  {
    route: "/boson/tasks/:task_name",
    feature_highlight: "boson-task-detail-view-queue",
    spotlight: "task-detail-view-queue",
    replay: false,
  },
  {
    route: "/boson/tasks/:task_name",
    feature_highlight: "boson-task-detail-view-runs",
    spotlight: "task-detail-view-runs",
    replay: false,
  },
  {
    route: "/boson/tasks/:task_name/config",
    feature_highlight: "boson-task-config-intro",
    spotlight: "boson-task-config",
    replay: false,
  },
  {
    route: "/boson/tasks/:task_name/config",
    feature_highlight: "boson-task-config-pool",
    spotlight: "task-config-pool",
    replay: false,
  },
  {
    route: "/boson/tasks/:task_name/config",
    feature_highlight: "boson-task-config-priority",
    spotlight: "task-config-priority",
    replay: false,
  },
  {
    route: "/boson/tasks/:task_name/config",
    feature_highlight: "boson-task-config-retry",
    spotlight: "boson-task-config-retry",
    replay: false,
  },
  {
    route: "/boson/tasks/:task_name/config",
    feature_highlight: "boson-task-config-cancel",
    spotlight: "task-config-cancel",
    replay: false,
  },
  {
    route: "/boson/tasks/:task_name/config",
    feature_highlight: "boson-task-config-save",
    spotlight: "task-config-save",
    replay: false,
  },
  {
    route: "/boson/queue",
    feature_highlight: "boson-queue-intro",
    spotlight: "boson-queue",
    replay: false,
  },
  {
    route: "/boson/queue",
    feature_highlight: "boson-queue-filter",
    spotlight: "queue-status-filter",
    replay: false,
  },
  {
    route: "/boson/queue",
    feature_highlight: "boson-queue-table",
    spotlight: "boson-queue-data-table",
    replay: false,
  },
  {
    route: "/boson/queue",
    feature_highlight: "boson-queue-open-runs",
    spotlight: "boson-queue-data-table",
    replay: false,
  },
  {
    route: "/boson/queue",
    feature_highlight: "boson-queue-cancel",
    spotlight: "boson-queue-cancel-hint",
    replay: false,
  },
  {
    route: "/boson/runs",
    feature_highlight: "boson-runs-intro",
    spotlight: "boson-runs",
    replay: false,
  },
  {
    route: "/boson/runs",
    feature_highlight: "boson-runs-table",
    spotlight: "boson-runs-data-table",
    replay: false,
  },
  {
    route: "/boson/runs",
    feature_highlight: "boson-runs-open",
    spotlight: "boson-runs-data-table",
    replay: false,
  },
  {
    route: "/boson/runs/:id",
    feature_highlight: "boson-run-detail-info",
    spotlight: "boson-run-detail-info",
    replay: false,
  },
  {
    route: "/boson/runs/:id",
    feature_highlight: "boson-run-detail-job",
    spotlight: "boson-run-detail-job",
    replay: false,
  },
  {
    route: "/boson/runs/:id",
    feature_highlight: "boson-run-detail-timing",
    spotlight: "boson-run-detail-timing",
    replay: false,
  },
  {
    route: "/boson/runs/:id",
    feature_highlight: "boson-run-detail-error",
    spotlight: "boson-run-detail-error",
    replay: false,
  },
] as const;

export async function seedAuth(
  page: Page,
  auth: SeedAuthKind,
  opts?: { refreshJob?: boolean; help_tour?: boolean },
) {
  const helpTour = opts?.help_tour ?? false;
  await page.addInitScript(
    ([enableTour, seenSteps]) => {
      try {
        if (enableTour) {
          if (!sessionStorage.getItem("uf.help.e2e_tour_cleared")) {
            localStorage.removeItem("uf.help.tour_steps");
            sessionStorage.setItem("uf.help.e2e_tour_cleared", "1");
          }
          return;
        }
        localStorage.setItem("uf.help.tour_steps", JSON.stringify(seenSteps));
      } catch {
        /* ignore */
      }
    },
    [helpTour, BOSON_HELP_STEPS_SEEN] as const,
  );

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

/**
 * Wait for Orbital boot overlay to finish and hydrate to mark the document ready.
 */
export async function waitForHydrated(page: Page, timeoutMs = 240_000) {
  await expect
    .poll(
      async () =>
        page.evaluate(() => {
          const html = document.documentElement;
          if (html.getAttribute("data-orbital-boot-state") === "error") {
            return "error";
          }
          if (html.getAttribute("data-orbital-hydrated") === "true") {
            return "ready";
          }
          return "loading";
        }),
      { timeout: timeoutMs },
    )
    .not.toBe("error");
  await expect
    .poll(
      async () =>
        page.evaluate(
          () => document.documentElement.getAttribute("data-orbital-hydrated") === "true",
        ),
      { timeout: timeoutMs },
    )
    .toBe(true);
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
