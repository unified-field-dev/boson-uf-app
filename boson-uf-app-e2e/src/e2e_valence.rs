//! Process-wide Valence + Higgs + MemQueue Boson for Playwright.
#![allow(dead_code)]

use std::sync::{Arc, Mutex, OnceLock};

use boson_backend_mem::MemQueueBackend;
use boson_coordinator::{BosonCoordinatorBackend, CoordinatorAdapter};
use boson_core::{
    ExecutionContext, JsonExecutionContextFactory, QueueBackend, QueueRouter, Run, RunStatus,
};
use boson_runtime::{Boson, TaskDescriptor, TaskRegistry};
use chrono::Utc;
use gauge::manifest_sync::{
    sync_permission_manifests, PermissionDomainInput, PermissionInput, PermissionManifestInput,
};
use gauge::service;
use gauge::super_user::SUPER_USER_GROUP_NAME;
use higgs::actor_policy::external_actor_json_policy;
use higgs::{HiggsConfig, HiggsValenceFactory};
use valence::{
    register_backend_logical_names, router_key, Actor, DatabaseBackend, DatabaseRouter,
    InMemoryBackend, Model, RegisterBackendLogicalNamesOptions, RouterValenceFactory,
    RouterValenceFactoryConfig, Valence, ValenceFactory, MEM_ENGINE_ID, SQLITE_ENGINE_ID,
};

struct E2eState {
    router: Arc<DatabaseRouter>,
    higgs: Arc<HiggsConfig>,
    boson_backend: Arc<dyn BosonCoordinatorBackend>,
    queue: Arc<MemQueueBackend>,
    default_backend_key: String,
    fixtures: Mutex<FixtureIds>,
}

/// Stable fixture ids exposed to seed JSON / Playwright.
#[derive(Clone, Debug, Default)]
pub struct FixtureIds {
    pub task_name: String,
    pub job_id: String,
    pub run_id: String,
}

static E2E_STATE: OnceLock<Arc<E2eState>> = OnceLock::new();

/// Lab task name registered into the in-process registry.
pub const E2E_TASK_NAME: &str = "e2e_echo";

fn e2e_echo_invoke(
    _ctx: Box<dyn ExecutionContext>,
    _params: serde_json::Value,
) -> std::pin::Pin<Box<dyn std::future::Future<Output = boson_core::Result<()>> + Send + 'static>> {
    Box::pin(async { Ok(()) })
}

fn e2e_task_registry() -> TaskRegistry {
    let mut registry = TaskRegistry::new();
    let desc: &'static TaskDescriptor = Box::leak(Box::new(TaskDescriptor::new(
        E2E_TASK_NAME,
        e2e_echo_invoke,
    )));
    registry.register(desc);
    registry
}

struct HiggsFactory(RouterValenceFactory);

impl HiggsValenceFactory for HiggsFactory {
    fn build(&self, actor_json: &serde_json::Value) -> anyhow::Result<Valence> {
        self.0.build(actor_json).map_err(|e| anyhow::anyhow!("{e}"))
    }
}

fn prepare_env() {
    valence::deletion::register_noop_deletion_dispatcher_for_tests();
    valence::clear_for_test();
    // SAFETY: host boot only.
    unsafe {
        if std::env::var_os("VALENCE_OWNERSHIP_UNIFIED_FETCH").is_none() {
            std::env::set_var("VALENCE_OWNERSHIP_UNIFIED_FETCH", "0");
        }
    }
}

async fn seed_user(id: &str, email_verified: bool, valence: &Valence) {
    let now = Utc::now();
    let confirmed_at = email_verified.then_some(now);
    let user = lepton::generated::User::new(
        Some(lepton::generated::UserUserType::Person),
        Some("e2e-password-hash".to_string()),
        Some(lepton::generated::UserStatus::Active),
        None,
        None,
        confirmed_at,
        None,
        None,
        now,
        now,
    )
    .expect("build user");
    lepton::generated::User::upsert(id, user, valence)
        .await
        .expect("upsert user");
}

async fn seed_super_user_with_member(system: &Valence, member_user_id: &str) {
    let super_group = gauge::generated::PermissionGroup::new(
        SUPER_USER_GROUP_NAME.to_string(),
        Some("super users".to_string()),
        Utc::now(),
        Utc::now(),
    )
    .expect("build super user group");
    let created =
        gauge::generated::PermissionGroup::upsert("super_user_group", super_group, system)
            .await
            .expect("upsert super user group");

    let member = lepton::generated::User::get(member_user_id, system)
        .await
        .expect("query member")
        .expect("member exists");
    let principal = gauge::generated::PermissionUserPrincipal::upsert(
        &format!("user:{member_user_id}"),
        gauge::generated::PermissionUserPrincipal::new(
            member.id().expect("member id").clone(),
            member_user_id.to_string(),
        )
        .expect("new principal"),
        system,
    )
    .await
    .expect("upsert principal");
    created
        .relate_to_owner_record(principal.id().expect("principal id"), system)
        .await
        .expect("relate super owner");
    created
        .relate_to_member_record(principal.id().expect("principal id"), system)
        .await
        .expect("relate super member");
}

async fn demote_admin_from_super_user(system: &Valence) {
    let Some(super_group) = gauge::generated::PermissionGroup::get("super_user_group", system)
        .await
        .expect("get super user group")
    else {
        return;
    };
    let Some(principal) = gauge::generated::PermissionUserPrincipal::get("user:admin", system)
        .await
        .expect("get admin principal")
    else {
        return;
    };
    let pid = principal.id().expect("principal id").clone();
    let _ = super_group.unrelate_from_member_record(&pid, system).await;
    let _ = super_group.unrelate_from_owner_record(&pid, system).await;
}

fn boson_admin_manifest() -> PermissionManifestInput {
    PermissionManifestInput {
        app_id: "boson".into(),
        domains: vec![PermissionDomainInput {
            key: "boson".into(),
            name: "Boson".into(),
            description: "Boson background-work administration".into(),
            permissions: vec![PermissionInput {
                name: "BosonAdmin".into(),
                description: "Administer Boson job cancellation and task configuration".into(),
            }],
        }],
    }
}

async fn grant_boson_admin(admin_ctx: &Valence, user_id: &str) {
    let perms = service::list_permissions(admin_ctx, None)
        .await
        .expect("list permissions");
    let boson_admin = perms
        .into_iter()
        .find(|p| p.name == "BosonAdmin")
        .expect("BosonAdmin after sync");
    service::grant_permission_to_user(&boson_admin.id, user_id, admin_ctx)
        .await
        .expect("grant BosonAdmin");
}

async fn bootstrap_boson_fixtures(
    coordinator: &dyn BosonCoordinatorBackend,
    queue: &MemQueueBackend,
) -> anyhow::Result<FixtureIds> {
    let job_id = coordinator
        .enqueue(
            E2E_TASK_NAME,
            serde_json::json!({"System": {"operation": "e2e_seed"}}),
            serde_json::json!({}),
            None,
        )
        .await?;

    let mut run = Run::new(&job_id, E2E_TASK_NAME, 1);
    run.status = RunStatus::Success;
    run.finished_at = Some(Utc::now());
    run.duration_ms = Some(12);
    queue.upsert_run(&run).await?;

    Ok(FixtureIds {
        task_name: E2E_TASK_NAME.into(),
        job_id,
        run_id: run.run_id,
    })
}

/// Build shared Valence/Higgs/Boson once and seed baseline fixtures.
pub async fn init_e2e_valence() {
    if E2E_STATE.get().is_some() {
        return;
    }

    prepare_env();

    let backend: Arc<dyn DatabaseBackend> = Arc::new(InMemoryBackend::new());
    let mut router = DatabaseRouter::new();
    register_backend_logical_names(
        &mut router,
        Arc::clone(&backend),
        gauge::embedded_surreal::EMBEDDED_SURREAL_LOGICAL_NAMES,
        RegisterBackendLogicalNamesOptions {
            register_alias_engine_id: Some(SQLITE_ENGINE_ID),
        },
    );
    router.register(
        router_key(gauge::embedded_surreal::LOGICAL_NAME, SQLITE_ENGINE_ID),
        Arc::clone(&backend),
    );
    let router = Arc::new(router);
    let default_key = router_key(gauge::embedded_surreal::LOGICAL_NAME, MEM_ENGINE_ID);

    let system = Valence::builder()
        .database_router(Arc::clone(&router))
        .default_backend_key(default_key.clone())
        .with_actor(Actor::System {
            operation: "e2e_boson_host".into(),
        })
        .build()
        .expect("e2e Valence");

    seed_user("admin", true, &system).await;
    seed_user("outsider", true, &system).await;
    seed_user("unverified", false, &system).await;
    seed_super_user_with_member(&system, "admin").await;

    sync_permission_manifests(&system, &[boson_admin_manifest()])
        .await
        .expect("sync BosonAdmin manifest");

    let admin_ctx = system.with_actor(Actor::User {
        user_id: "admin".to_string(),
    });
    grant_boson_admin(&admin_ctx, "admin").await;
    grant_boson_admin(&admin_ctx, "unverified").await;
    demote_admin_from_super_user(&system).await;

    let queue = Arc::new(MemQueueBackend::new());
    let queue_backend: Arc<dyn QueueBackend> = Arc::clone(&queue) as Arc<dyn QueueBackend>;
    QueueRouter::set_global(QueueRouter::with_default(Arc::clone(&queue_backend)));

    let boson = Arc::new(
        Boson::builder()
            .queue_backend_from_global()
            .execution_context_factory(JsonExecutionContextFactory)
            .registry(std::sync::Arc::new(e2e_task_registry()))
            .without_worker()
            .build_manual()
            .expect("e2e Boson")
            .0,
    );
    boson_runtime::configure((*boson).clone());
    let boson_backend: Arc<dyn BosonCoordinatorBackend> =
        Arc::new(CoordinatorAdapter::new(Arc::clone(&boson)));

    let fixtures = bootstrap_boson_fixtures(boson_backend.as_ref(), queue.as_ref())
        .await
        .expect("bootstrap boson fixtures");

    let factory: Arc<dyn HiggsValenceFactory> = Arc::new(HiggsFactory(RouterValenceFactory::new(
        Arc::clone(&router),
        RouterValenceFactoryConfig::new(default_key.clone())
            .actor_json_policy(external_actor_json_policy()),
    )));
    // Lab host provides the coordinator via Leptos `provide_context` (see main.rs).
    // Skip HiggsConfig::boson to avoid boson-coordinator version skew with higgs/boson.
    let higgs = Arc::new(
        HiggsConfig::builder()
            .valence_factory_arc(factory)
            .build()
            .expect("e2e HiggsConfig"),
    );

    let state = Arc::new(E2eState {
        router,
        higgs,
        boson_backend,
        queue,
        default_backend_key: default_key,
        fixtures: Mutex::new(fixtures),
    });
    let _ = E2E_STATE.set(state);
}

fn state() -> Arc<E2eState> {
    E2E_STATE
        .get()
        .expect("init_e2e_valence must run first")
        .clone()
}

pub fn e2e_router() -> Arc<DatabaseRouter> {
    Arc::clone(&state().router)
}

pub fn e2e_higgs_config() -> Arc<HiggsConfig> {
    Arc::clone(&state().higgs)
}

pub fn e2e_boson_backend() -> Arc<dyn BosonCoordinatorBackend> {
    Arc::clone(&state().boson_backend)
}

pub fn e2e_fixtures() -> FixtureIds {
    state().fixtures.lock().expect("fixtures").clone()
}

pub fn store_fixtures(fixtures: FixtureIds) {
    *state().fixtures.lock().expect("fixtures") = fixtures;
}

pub fn e2e_queue() -> Arc<MemQueueBackend> {
    Arc::clone(&state().queue)
}

/// Re-enqueue a fresh queued job (and optional run) for isolated cancel specs.
pub async fn refresh_queue_job() -> anyhow::Result<FixtureIds> {
    let backend = e2e_boson_backend();
    let queue = e2e_queue();
    let mut fixtures = e2e_fixtures();
    let job_id = backend
        .enqueue(
            E2E_TASK_NAME,
            serde_json::json!({"System": {"operation": "e2e_refresh"}}),
            serde_json::json!({}),
            None,
        )
        .await?;
    fixtures.job_id = job_id;
    store_fixtures(fixtures.clone());
    let _ = queue;
    Ok(fixtures)
}

pub fn e2e_system_valence() -> Valence {
    Valence::builder()
        .database_router(e2e_router())
        .default_backend_key(state().default_backend_key.clone())
        .with_actor(Actor::System {
            operation: "e2e_seed".into(),
        })
        .build()
        .expect("system valence")
}
