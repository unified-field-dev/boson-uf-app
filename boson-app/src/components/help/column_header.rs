use leptos::prelude::*;
use orbital::components::Caption1;
use orbital::primitives::{InfoLabel, InfoLabelInfo};

fn column_test_id(label: &str) -> String {
    format!(
        "boson-help-col-{}",
        label.to_ascii_lowercase().replace(' ', "-")
    )
}

/// Table column header with an optional info popover.
#[component]
pub fn BosonHelpColumnHeader(
    /// Label text.
    label: &'static str,
    /// Supplementary info/help content.
    #[prop(optional)] info: Option<AnyView>,
) -> impl IntoView {
    view! {
        {if let Some(info_view) = info {
            view! {
                <div data-testid=column_test_id(label)>
                    <InfoLabel>
                        <Caption1>{label}</Caption1>
                        <InfoLabelInfo slot>
                            {info_view}
                        </InfoLabelInfo>
                    </InfoLabel>
                </div>
            }.into_any()
        } else {
            view! { <Caption1>{label}</Caption1> }.into_any()
        }}
    }
}
