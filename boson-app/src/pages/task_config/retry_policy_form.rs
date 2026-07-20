use leptos::prelude::*;
use orbital::components::{Card, FormHint, SpacingSize};
use orbital::primitives::*;

use crate::components::{
    backoff_multiplier_help, initial_delay_hint, max_attempts_hint, max_delay_help,
    retry_policy_help, BosonHelpCardHeader,
};

/// Retry policy form section (max attempts, delays, backoff).
#[component]
pub fn RetryPolicyForm(
    /// Two-way signal holding the max attempts.
    max_attempts_str: RwSignal<String>,
    /// Two-way signal holding the base delay milliseconds.
    base_delay_ms_str: RwSignal<String>,
    /// Two-way signal holding the max delay milliseconds.
    max_delay_ms_str: RwSignal<String>,
    /// Two-way signal holding the backoff multiplier.
    backoff_multiplier_str: RwSignal<String>,
) -> impl IntoView {
    view! {
        <Card>
            <BosonHelpCardHeader
                title="Retry Policy"
                description="Tune how failed runs are retried before the job is marked failed."
                info=retry_policy_help()
            />
            <Flex vertical=true gap=SpacingSize::Size160.flex_gap() padding=SpacingSize::Size160.inset()>
                <Flex vertical=true gap=SpacingSize::Size40.flex_gap()>
                    <Label>"Max Attempts"</Label>
                    <Input
                        bind=max_attempts_str
                        appearance=InputAppearance {
                            input_type: Signal::from(InputType::Number),
                            ..Default::default()
                        }
                    />
                    <FormHint>{max_attempts_hint()}</FormHint>
                </Flex>
                <Flex vertical=true gap=SpacingSize::Size40.flex_gap()>
                    <Label>"Initial Delay (ms)"</Label>
                    <Input
                        bind=base_delay_ms_str
                        appearance=InputAppearance {
                            input_type: Signal::from(InputType::Number),
                            ..Default::default()
                        }
                    />
                    <FormHint>{initial_delay_hint()}</FormHint>
                </Flex>
                <Flex vertical=true gap=SpacingSize::Size40.flex_gap()>
                    <InfoLabel>
                        <Label>"Max Delay (ms)"</Label>
                        <InfoLabelInfo slot>
                            {max_delay_help()}
                        </InfoLabelInfo>
                    </InfoLabel>
                    <Input
                        bind=max_delay_ms_str
                        appearance=InputAppearance {
                            input_type: Signal::from(InputType::Number),
                            ..Default::default()
                        }
                    />
                </Flex>
                <Flex vertical=true gap=SpacingSize::Size40.flex_gap()>
                    <InfoLabel>
                        <Label>"Backoff Multiplier"</Label>
                        <InfoLabelInfo slot>
                            {backoff_multiplier_help()}
                        </InfoLabelInfo>
                    </InfoLabel>
                    <Input
                        bind=backoff_multiplier_str
                        appearance=InputAppearance {
                            input_type: Signal::from(InputType::Number),
                            ..Default::default()
                        }
                    />
                </Flex>
            </Flex>
        </Card>
    }
}
