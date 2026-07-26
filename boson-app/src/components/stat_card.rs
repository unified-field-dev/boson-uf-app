use leptos::prelude::*;
use orbital::components::{Caption1, SpacingSize, StatCard, StatCardVariant, Title2};
use orbital::primitives::{Card, Flex, FlexAlign, Icon, InfoLabel, InfoLabelInfo};
use turf::inline_style_sheet_values;

/// Stat card with optional InfoLabel on the metric label (when StatCard label prop is insufficient).
#[component]
pub fn BosonHelpStatCard(
    /// Label text.
    label: &'static str,
    /// Reactive signal for the current value.
    #[prop(into)]
    value: Signal<String>,
    /// Icon to display.
    #[prop(optional)]
    icon: Option<icondata_core::Icon>,
    /// Visual variant to render.
    #[prop(optional)]
    variant: Option<StatCardVariant>,
    /// Supplementary info/help content for the label.
    #[prop(optional)]
    label_info: Option<AnyView>,
) -> impl IntoView {
    match label_info {
        None => {
            let card_variant = variant.unwrap_or_default();
            icon.map_or_else(
                || {
                    view! {
                        <StatCard label=label value=value variant=card_variant />
                    }
                    .into_any()
                },
                |icon_data| {
                    view! {
                        <StatCard label=label value=value icon=icon_data variant=card_variant />
                    }
                    .into_any()
                },
            )
        }
        Some(info_view) => {
            let variant = variant.unwrap_or_default();
            let (style_sheet, class_names) = inline_style_sheet_values! {
                .Card {
                    min-width: 140px;
                    flex: 1;
                }
                .Label {
                    color: var(--orb-color-text-tertiary);
                }
                .ValueSuccess {
                    color: var(--orb-color-status-success-fg);
                }
                .ValueDanger {
                    color: var(--orb-color-status-danger-fg);
                }
                .ValueWarning {
                    color: var(--orb-color-status-warning-fg);
                }
            };

            let value_class = match variant {
                StatCardVariant::Default => String::new(),
                StatCardVariant::Success => class_names.value_success.to_string(),
                StatCardVariant::Danger => class_names.value_danger.to_string(),
                StatCardVariant::Warning => class_names.value_warning.to_string(),
            };

            view! {
                <style>{style_sheet}</style>
                <Card class=class_names.card>
                    <Flex
                        vertical=true
                        gap=SpacingSize::Size80.flex_gap()
                        padding=SpacingSize::Size160.inset()
                    >
                        <Flex align=FlexAlign::Center gap=SpacingSize::Size80.flex_gap()>
                            {icon.map(|i| view! { <Icon icon=i /> })}
                            <InfoLabel>
                                <Caption1 class=class_names.label>{label}</Caption1>
                                <InfoLabelInfo slot>
                                    {info_view}
                                </InfoLabelInfo>
                            </InfoLabel>
                        </Flex>
                        <Title2 class=value_class>{move || value.get()}</Title2>
                    </Flex>
                </Card>
            }
            .into_any()
        }
    }
}
