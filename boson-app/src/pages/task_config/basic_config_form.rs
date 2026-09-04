use leptos::prelude::*;
use orbital::components::{Card, FormHint, SpacingSize};
use orbital::primitives::{
    Flex, InfoLabel, InfoLabelInfo, Input, InputAppearance, InputType, Label, Select,
};

use crate::components::{basic_config_help, pool_field_help, BosonHelpCardHeader};
use crate::server::GluonPoolPickRow;

/// Basic configuration form section (pool and priority).
#[component]
#[allow(clippy::needless_pass_by_value)]
pub fn TaskConfigForm(
    /// Two-way signal holding the resource pool identifier.
    pool: RwSignal<String>,
    /// Two-way signal holding the priority.
    priority_str: RwSignal<String>,
    /// List of pool options.
    #[prop(default = Vec::new())]
    pool_options: Vec<GluonPoolPickRow>,
) -> impl IntoView {
    let show_select = !pool_options.is_empty();
    let opts_for_select = pool_options;

    view! {
        <Card>
            <BosonHelpCardHeader
                title="Basic Configuration"
                description="Choose where this task runs and its default queue priority."
                info=basic_config_help()
            />
            <Flex vertical=true gap=SpacingSize::Size160.flex_gap() padding=SpacingSize::Size160.inset()>
                <Flex vertical=true gap=SpacingSize::Size40.flex_gap()>
                    <InfoLabel>
                        <Label>"Pool"</Label>
                        <InfoLabelInfo slot>
                            {pool_field_help()}
                        </InfoLabelInfo>
                    </InfoLabel>
                    {move || {
                        if show_select {
                            let opts = opts_for_select.clone();
                            let opts_hint = opts.clone();
                            view! {
                                <div data-testid="task-config-pool">
                                    <Select bind=pool>
                                        <For
                                            each=move || opts.clone()
                                            key=|o| o.id.clone()
                                            children=move |o| {
                                                let v = o.id.clone();
                                                let lbl = o.label;
                                                view! { <option value=v>{lbl}</option> }
                                            }
                                        />
                                    </Select>
                                </div>
                                <FormHint>{move || {
                                    let p = pool.get();
                                    opts_hint
                                        .iter()
                                        .find(|o| o.id == p)
                                        .map(|o| o.detail.clone())
                                        .unwrap_or_default()
                                }}</FormHint>
                            }
                            .into_any()
                        } else {
                            view! {
                                <Input bind=pool />
                                <FormHint>"Default: \"global\". Configure Gluon virtual pools to pick from a list."</FormHint>
                            }
                            .into_any()
                        }
                    }}
                </Flex>
                <Flex vertical=true gap=SpacingSize::Size40.flex_gap()>
                    <Label>"Default Priority"</Label>
                    <Input
                        bind=priority_str
                        appearance=InputAppearance {
                            input_type: Signal::from(InputType::Number),
                            ..Default::default()
                        }
                    />
                    <FormHint>"Default: 1. Lower value = higher priority (runs sooner)"</FormHint>
                </Flex>
            </Flex>
        </Card>
    }
}
