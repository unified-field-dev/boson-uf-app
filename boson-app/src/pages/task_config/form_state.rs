use leptos::prelude::*;
use leptos::server_fn::ServerFnError;
use leptos::task::spawn_local_scoped;

use crate::server::{update_task_config, RetryPolicyDto, TaskConfigDto, UpdateTaskConfigRequest};

/// Form field signals and save handler for task configuration.
pub struct TaskConfigFormState {
    pub priority: RwSignal<i32>,
    pub pool: RwSignal<String>,
    pub max_attempts: RwSignal<u32>,
    pub base_delay_ms: RwSignal<u64>,
    pub backoff_multiplier: RwSignal<f64>,
    pub max_delay_ms: RwSignal<u64>,
    pub priority_str: RwSignal<String>,
    pub max_attempts_str: RwSignal<String>,
    pub base_delay_ms_str: RwSignal<String>,
    pub max_delay_ms_str: RwSignal<String>,
    pub backoff_multiplier_str: RwSignal<String>,
    pub save_pending: RwSignal<bool>,
    pub save_error: RwSignal<Option<String>>,
}

impl TaskConfigFormState {
    pub fn new() -> Self {
        let priority = RwSignal::new(1i32);
        let pool = RwSignal::new("global".to_string());
        let max_attempts = RwSignal::new(3u32);
        let base_delay_ms = RwSignal::new(1000u64);
        let backoff_multiplier = RwSignal::new(2.0f64);
        let max_delay_ms = RwSignal::new(300_000u64);

        let priority_str = RwSignal::new(priority.get_untracked().to_string());
        let max_attempts_str = RwSignal::new(max_attempts.get_untracked().to_string());
        let base_delay_ms_str = RwSignal::new(base_delay_ms.get_untracked().to_string());
        let max_delay_ms_str = RwSignal::new(max_delay_ms.get_untracked().to_string());
        let backoff_multiplier_str = RwSignal::new(backoff_multiplier.get_untracked().to_string());

        Self {
            priority,
            pool,
            max_attempts,
            base_delay_ms,
            backoff_multiplier,
            max_delay_ms,
            priority_str,
            max_attempts_str,
            base_delay_ms_str,
            max_delay_ms_str,
            backoff_multiplier_str,
            save_pending: RwSignal::new(false),
            save_error: RwSignal::new(None),
        }
    }

    pub fn wire_parse_effects(&self) {
        let priority = self.priority;
        let priority_str = self.priority_str;
        Effect::new(move || {
            if let Ok(n) = priority_str.get().parse::<i32>() {
                priority.set(n);
            }
        });

        let max_attempts = self.max_attempts;
        let max_attempts_str = self.max_attempts_str;
        Effect::new(move || {
            if let Ok(n) = max_attempts_str.get().parse::<u32>() {
                max_attempts.set(n);
            }
        });

        let base_delay_ms = self.base_delay_ms;
        let base_delay_ms_str = self.base_delay_ms_str;
        Effect::new(move || {
            if let Ok(n) = base_delay_ms_str.get().parse::<u64>() {
                base_delay_ms.set(n);
            }
        });

        let max_delay_ms = self.max_delay_ms;
        let max_delay_ms_str = self.max_delay_ms_str;
        Effect::new(move || {
            if let Ok(n) = max_delay_ms_str.get().parse::<u64>() {
                max_delay_ms.set(n);
            }
        });

        let backoff_multiplier = self.backoff_multiplier;
        let backoff_multiplier_str = self.backoff_multiplier_str;
        Effect::new(move || {
            if let Ok(n) = backoff_multiplier_str.get().parse::<f64>() {
                backoff_multiplier.set(n);
            }
        });
    }

    pub fn wire_populate_effect(&self, config_res: Resource<Result<TaskConfigDto, ServerFnError>>) {
        let priority = self.priority;
        let priority_str = self.priority_str;
        let pool = self.pool;
        let max_attempts = self.max_attempts;
        let max_attempts_str = self.max_attempts_str;
        let base_delay_ms = self.base_delay_ms;
        let base_delay_ms_str = self.base_delay_ms_str;
        let backoff_multiplier = self.backoff_multiplier;
        let backoff_multiplier_str = self.backoff_multiplier_str;
        let max_delay_ms = self.max_delay_ms;
        let max_delay_ms_str = self.max_delay_ms_str;

        Effect::new(move |_| {
            let _ = config_res.get();
            if let Some(Ok(c)) = config_res.get() {
                priority.set(c.priority);
                priority_str.set(c.priority.to_string());
                pool.set(c.pool);
                max_attempts.set(c.retry_policy.max_attempts);
                max_attempts_str.set(c.retry_policy.max_attempts.to_string());
                base_delay_ms.set(c.retry_policy.base_delay_ms);
                base_delay_ms_str.set(c.retry_policy.base_delay_ms.to_string());
                backoff_multiplier.set(c.retry_policy.backoff_multiplier);
                backoff_multiplier_str.set(c.retry_policy.backoff_multiplier.to_string());
                max_delay_ms.set(c.retry_policy.max_delay_ms);
                max_delay_ms_str.set(c.retry_policy.max_delay_ms.to_string());
            }
        });
    }

    pub fn save_callback(
        &self,
        task_name: Memo<String>,
        config_res: Resource<Result<TaskConfigDto, ServerFnError>>,
    ) -> Callback<leptos::ev::MouseEvent> {
        let save_pending = self.save_pending;
        let save_error = self.save_error;
        let priority = self.priority;
        let pool = self.pool;
        let max_attempts = self.max_attempts;
        let base_delay_ms = self.base_delay_ms;
        let backoff_multiplier = self.backoff_multiplier;
        let max_delay_ms = self.max_delay_ms;

        Callback::new(move |_| {
            let name = task_name.get();
            if name.is_empty() {
                return;
            }
            save_pending.set(true);
            save_error.set(None);
            let req = UpdateTaskConfigRequest {
                priority: Some(priority.get()),
                pool: Some(pool.get()),
                retry_policy: Some(RetryPolicyDto {
                    max_attempts: max_attempts.get(),
                    base_delay_ms: base_delay_ms.get(),
                    backoff_multiplier: backoff_multiplier.get(),
                    max_delay_ms: max_delay_ms.get(),
                }),
            };
            spawn_local_scoped(async move {
                match update_task_config(name, req).await {
                    Ok(_) => {
                        save_pending.set(false);
                        config_res.refetch();
                    }
                    Err(e) => {
                        save_pending.set(false);
                        save_error.set(Some(e.to_string()));
                    }
                }
            });
        })
    }
}

pub fn use_task_config_form(
    config_res: Resource<Result<TaskConfigDto, ServerFnError>>,
) -> TaskConfigFormState {
    let form = TaskConfigFormState::new();
    form.wire_parse_effects();
    form.wire_populate_effect(config_res);
    form
}
