//! Product surface contracts for boson-app (sibling crate).
//!
//! Lives under `boson-backend` so CI can gate route/testid/auth/admin needles
//! without compiling Orbital/turf UI when host pins churn. Pattern matches
//! photon-uf-app `photon-backend/tests/product_surface.rs`, gauge
//! `gauge/tests/product_surface.rs`, and lepton-uf-app
//! `lepton-shell/tests/product_surface.rs`.

use std::fs;
use std::path::PathBuf;

fn workspace_root() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("..")
}

fn read_app(rel: &str) -> String {
    let path = workspace_root().join("boson-app").join("src").join(rel);
    fs::read_to_string(&path).unwrap_or_else(|e| panic!("read {}: {e}", path.display()))
}

#[test]
fn boson_routes_mount_happy_path() {
    let lib = read_app("lib.rs");
    for needle in [
        r#"path!("boson")"#,
        r#"path!("")"#,
        r#"path!("tasks")"#,
        r#"path!("tasks/:task_name")"#,
        r#"path!("tasks/:task_name/config")"#,
        r#"path!("queue")"#,
        r#"path!("runs")"#,
        r#"path!("runs/:id")"#,
        "BosonLayoutRouteView",
        "id: \"boson\"",
        "route_path: \"/boson\"",
        "permission_manifest: permissions::BosonPermission",
    ] {
        assert!(
            lib.contains(needle),
            "BosonRoutes / uf_app missing `{needle}`"
        );
    }
}

#[test]
fn boson_routes_drop_leaf_sad_path() {
    let lib = read_app("lib.rs");
    for needle in [
        r#"path!("tasks/:task_name")"#,
        r#"path!("tasks/:task_name/config")"#,
        r#"path!("queue")"#,
        r#"path!("runs/:id")"#,
    ] {
        assert!(
            lib.contains(needle),
            "removing `{needle}` drops a Boson ops funnel entry"
        );
    }
    assert!(
        !lib.contains("unimplemented!"),
        "BosonRoutes must not ship unimplemented placeholders"
    );
}

#[test]
fn uf_app_wrong_id_sad_path() {
    let lib = read_app("lib.rs");
    assert!(
        lib.contains("id: \"boson\""),
        "wrong uf_app id breaks Orbital host registration"
    );
    assert!(
        !lib.contains("id: \"boson-app\""),
        "uf_app id must stay `boson` (product route id), not crate name boson-app"
    );
}

#[test]
fn layout_auth_gate_and_nav_happy_path() {
    let layout = read_app("layout.rs");
    for needle in [
        "boson-app-root",
        "RequireAuthenticated",
        "Outlet",
        "nav-boson-dashboard",
        "nav-boson-tasks",
        "nav-boson-queue",
        "nav-boson-runs",
        "AppBarUserMenu",
        "UnifiedFieldShellLayout",
    ] {
        assert!(
            layout.contains(needle),
            "BosonLayout missing contract `{needle}`"
        );
    }
}

#[test]
fn layout_drop_auth_guard_sad_path() {
    let layout = read_app("layout.rs");
    assert!(
        layout.contains("RequireAuthenticated") && layout.contains("<Outlet />"),
        "removing RequireAuthenticated opens /boson pages to anonymous sessions"
    );
}

#[test]
fn layout_missing_nav_sad_path() {
    let layout = read_app("layout.rs");
    for id in [
        "nav-boson-dashboard",
        "nav-boson-tasks",
        "nav-boson-queue",
        "nav-boson-runs",
    ] {
        assert!(
            layout.contains(id),
            "dropping `{id}` breaks operator left-nav contract"
        );
    }
}

#[test]
fn admin_mutators_require_boson_admin_happy_path() {
    let jobs = read_app("server/jobs.rs");
    let tasks = read_app("server/tasks.rs");
    let pools = read_app("server/gluon_pools.rs");
    let combined = format!("{jobs}\n{tasks}\n{pools}");

    for fn_name in [
        "cancel_job",
        "get_task_config",
        "update_task_config",
        "list_gluon_pools_for_boson_task_config",
    ] {
        assert!(
            combined.contains(fn_name),
            "server missing admin surface `{fn_name}`"
        );
    }
    let admin_attr = r#"permission = "BosonAdmin""#;
    assert!(
        combined.matches(admin_attr).count() >= 4,
        "cancel / task-config / pool server fns must carry BosonAdmin permission attribute"
    );
}

#[test]
fn admin_mutators_drop_boson_admin_sad_path() {
    let jobs = read_app("server/jobs.rs");
    let tasks = read_app("server/tasks.rs");
    let pools = read_app("server/gluon_pools.rs");
    let combined = format!("{jobs}\n{tasks}\n{pools}");
    let admin_attr = r#"permission = "BosonAdmin""#;
    assert!(
        combined.matches(admin_attr).count() >= 4,
        "dropping BosonAdmin from cancel/config/pools opens mutating ops without admin gate"
    );
    assert!(
        !combined.contains(r#"permission = "GaugeAdmin""#)
            && !combined.contains(r#"permission = "PhotonAdmin""#),
        "Boson admin mutators must not gate on GaugeAdmin or PhotonAdmin"
    );
}

#[test]
fn server_require_session_happy_path() {
    let helpers = read_app("server/helpers.rs");
    assert!(
        helpers.contains("fn require_session")
            && helpers.contains("Authentication is required")
            && helpers.contains("session_user_id()"),
        "helpers must fail closed without a session"
    );
    assert!(
        helpers.contains("Boson backend not in request context"),
        "missing Boson backend context must surface a typed ServerFnError message"
    );
    assert!(
        helpers.contains("fn require_email_verified")
            && helpers.contains("Email verification is required"),
        "helpers must fail closed without a verified email for task-config"
    );

    let dashboard = read_app("server/dashboard.rs");
    let jobs = read_app("server/jobs.rs");
    let tasks = read_app("server/tasks.rs");
    for (src, call_site) in [
        (dashboard.as_str(), "get_dashboard_stats"),
        (tasks.as_str(), "get_tasks"),
        (jobs.as_str(), "cancel_job"),
    ] {
        assert!(src.contains(call_site), "server missing `{call_site}`");
    }
}

#[test]
fn server_drop_require_session_on_get_tasks_sad_path() {
    let tasks = read_app("server/tasks.rs");
    let start = tasks.find("pub async fn get_tasks").expect("get_tasks");
    let body = &tasks[start..start + 350.min(tasks.len() - start)];
    assert!(
        body.contains("require_session(&ctx)?"),
        "get_tasks must call require_session before Boson IO"
    );

    let jobs = read_app("server/jobs.rs");
    let start = jobs.find("pub async fn cancel_job").expect("cancel_job");
    let body = &jobs[start..start + 450.min(jobs.len() - start)];
    assert!(
        body.contains("require_session(&ctx)?"),
        "cancel_job must call require_session before Boson IO"
    );
}

#[test]
fn task_config_email_gate_happy_path() {
    let lazy = read_app("lazy_routes.rs");
    assert!(
        lazy.contains("requires_email_verification=true") && lazy.contains("BosonTaskConfigPage"),
        "task config lazy route must require email verification"
    );

    let tasks = read_app("server/tasks.rs");
    let start = tasks
        .find("pub async fn get_task_config")
        .expect("get_task_config");
    let body = &tasks[start..start + 500.min(tasks.len() - start)];
    assert!(
        body.contains("require_email_verified().await?"),
        "get_task_config must mirror the UI email-verification gate"
    );
}

#[test]
fn task_config_drop_email_gate_sad_path() {
    let lazy = read_app("lazy_routes.rs");
    assert!(
        lazy.contains("requires_email_verification=true"),
        "dropping email verification on BosonVerifiedTaskConfigRoute opens config to unverified sessions"
    );
    let tasks = read_app("server/tasks.rs");
    let start = tasks
        .find("pub async fn update_task_config")
        .expect("update_task_config");
    let body = &tasks[start..start + 750.min(tasks.len() - start)];
    assert!(
        body.contains("require_email_verified().await?"),
        "update_task_config must keep require_email_verified"
    );
}

#[test]
fn index_pages_testid_and_list_bindings_happy_path() {
    let dashboard = read_app("pages/dashboard/mod.rs");
    for needle in ["boson-dashboard", "get_dashboard_stats", "get_tasks"] {
        assert!(
            dashboard.contains(needle),
            "BosonRootPage / dashboard missing `{needle}`"
        );
    }

    let tasks = read_app("pages/tasks/mod.rs");
    assert!(
        tasks.contains("boson-tasks"),
        "BosonTasksIndexPage missing boson-tasks testid"
    );

    let queue = read_app("pages/queue/mod.rs");
    for needle in ["boson-queue", "cancel_job"] {
        assert!(queue.contains(needle), "BosonQueuePage missing `{needle}`");
    }

    let runs = read_app("pages/runs/mod.rs");
    assert!(
        runs.contains("boson-runs"),
        "BosonRunsIndexPage missing boson-runs testid"
    );
}

#[test]
fn index_drop_dashboard_testid_sad_path() {
    let dashboard = read_app("pages/dashboard/mod.rs");
    assert!(
        dashboard.contains("data_testid=\"boson-dashboard\""),
        "dropping boson-dashboard breaks host / future Playwright parity"
    );
    let tasks = read_app("pages/tasks/mod.rs");
    assert!(
        tasks.contains("data_testid=\"boson-tasks\""),
        "dropping boson-tasks breaks host / future Playwright parity"
    );
    let queue = read_app("pages/queue/mod.rs");
    assert!(
        queue.contains("data_testid=\"boson-queue\""),
        "dropping boson-queue breaks host / future Playwright parity"
    );
    let runs = read_app("pages/runs/mod.rs");
    assert!(
        runs.contains("data_testid=\"boson-runs\""),
        "dropping boson-runs breaks host / future Playwright parity"
    );
}

#[test]
fn detail_pages_testid_and_bindings_happy_path() {
    let task = read_app("pages/task_detail/mod.rs");
    for needle in ["boson-task-detail", "get_task"] {
        assert!(
            task.contains(needle),
            "BosonTaskDetailPage missing `{needle}`"
        );
    }

    let config = read_app("pages/task_config/mod.rs");
    for needle in [
        "boson-task-config",
        "get_task_config",
        "list_gluon_pools_for_boson_task_config",
    ] {
        assert!(
            config.contains(needle),
            "BosonTaskConfigPage missing `{needle}`"
        );
    }

    let run = read_app("pages/run_detail/mod.rs");
    for needle in ["boson-run-detail", "get_run"] {
        assert!(
            run.contains(needle),
            "BosonRunDetailPage missing `{needle}`"
        );
    }
}

#[test]
fn detail_pages_missing_bindings_sad_path() {
    let task = read_app("pages/task_detail/mod.rs");
    assert!(task.contains("get_task"), "task detail must bind get_task");
    let config = read_app("pages/task_config/mod.rs");
    assert!(
        config.contains("get_task_config"),
        "task config must bind get_task_config"
    );
    let run = read_app("pages/run_detail/mod.rs");
    assert!(run.contains("get_run"), "run detail must bind get_run");
    assert!(
        !task.contains("unimplemented!")
            && !config.contains("unimplemented!")
            && !run.contains("unimplemented!"),
        "detail pages must not ship unimplemented placeholders"
    );
}

#[test]
fn permission_manifest_boson_admin_happy_path() {
    let perms = read_app("permissions.rs");
    for needle in [
        "domain_key = \"boson\"",
        "BosonAdmin",
        "UfPermissionManifest",
    ] {
        assert!(
            perms.contains(needle),
            "BosonPermission manifest missing `{needle}`"
        );
    }
}

#[test]
fn protected_boson_host_matches_uf_app_happy_path() {
    let host =
        fs::read_to_string(workspace_root().join("examples/protected-boson-host/src/main.rs"))
            .expect("protected-boson-host main.rs");
    for needle in [
        "\"app_id\": \"boson\"",
        "\"route_path\": \"/boson\"",
        "\"auth_gate\": \"RequireAuthenticated\"",
        "\"admin_permission\": \"BosonAdmin\"",
        "dashboard_stats",
    ] {
        assert!(
            host.contains(needle),
            "protected-boson-host missing contract `{needle}`"
        );
    }
    let lib = read_app("lib.rs");
    assert!(
        lib.contains("id: \"boson\"") && lib.contains("route_path: \"/boson\""),
        "host inventory must stay aligned with uf_app!"
    );
    let layout = read_app("layout.rs");
    assert!(
        layout.contains("RequireAuthenticated"),
        "host auth_gate must stay aligned with BosonLayout guard"
    );
    let perms = read_app("permissions.rs");
    assert!(
        perms.contains("BosonAdmin"),
        "host admin_permission must stay aligned with BosonPermission"
    );
}

#[test]
fn lazy_routes_wire_pages_happy_path() {
    let lazy = read_app("lazy_routes.rs");
    for needle in [
        "BosonRootPage",
        "BosonTasksIndexPage",
        "BosonTaskDetailPage",
        "BosonTaskConfigPage",
        "BosonQueuePage",
        "BosonRunsIndexPage",
        "BosonRunDetailPage",
        "BosonLayout",
    ] {
        assert!(
            lazy.contains(needle),
            "lazy_routes missing page wire `{needle}`"
        );
    }
}

#[test]
fn ops_path_helpers_encode_segments_happy_path() {
    let recent = read_app("pages/dashboard/recent_tasks_table.rs");
    let task_detail = read_app("pages/task_detail/mod.rs");
    let task_actions = read_app("components/tasks_data_table/actions.rs");
    let tasks_table = read_app("components/tasks_data_table/mod.rs");
    let runs_cols = read_app("components/runs_data_table/columns.rs");
    let runs_table = read_app("components/runs_data_table/mod.rs");
    let queue = read_app("components/queue_data_table/mod.rs");
    let run_info = read_app("pages/run_detail/run_info_grid.rs");
    for (label, src) in [
        ("recent_tasks_table", recent.as_str()),
        ("task_detail", task_detail.as_str()),
        ("task_actions", task_actions.as_str()),
        ("tasks_data_table", tasks_table.as_str()),
        ("runs_columns", runs_cols.as_str()),
        ("runs_data_table", runs_table.as_str()),
        ("queue_data_table", queue.as_str()),
        ("run_info_grid", run_info.as_str()),
    ] {
        assert!(
            src.contains("boson_backend::boson_")
                || src.contains("boson_task_path")
                || src.contains("boson_run_path")
                || src.contains("boson_task_config_path")
                || src.contains("boson_runs_job_filter_path"),
            "{label} must build detail hrefs via boson_backend path helpers"
        );
        assert!(
            !src.contains("crate::paths::task(")
                && !src.contains("crate::paths::run(")
                && !src.contains("crate::paths::tasks_config("),
            "{label} must not interpolate raw ids into orbital paths::*"
        );
    }
}

#[test]
fn ops_path_helpers_drop_encoding_sad_path() {
    let recent = read_app("pages/dashboard/recent_tasks_table.rs");
    assert!(
        recent.contains("boson_backend::boson_task_path"),
        "dropping boson_task_path reopens path-segment smuggling via task names"
    );
    let runs_cols = read_app("components/runs_data_table/columns.rs");
    assert!(
        runs_cols.contains("boson_backend::boson_run_path"),
        "dropping boson_run_path reopens path-segment smuggling via run ids"
    );
    let queue = read_app("components/queue_data_table/mod.rs");
    assert!(
        queue.contains("boson_backend::boson_runs_job_filter_path"),
        "dropping boson_runs_job_filter_path reopens query-value smuggling via job ids"
    );
}
