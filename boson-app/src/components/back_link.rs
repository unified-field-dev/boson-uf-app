use leptos::prelude::*;
use leptos_router::hooks::use_navigate;
use orbital::primitives::*;

/// Shared back navigation using Orbital Button (chronon job-create pattern).
#[component]
pub fn BosonBackLink(
    /// Link target.
    href: &'static str,
    /// Label text.
    label: &'static str,
    /// Optional data testid.
    #[prop(optional)] data_testid: Option<&'static str>,
) -> impl IntoView {
    let navigate = use_navigate();
    let navigate_store = StoredValue::new(navigate);

    let button = view! {
        <Button
            appearance=ButtonAppearance::Subtle
            icon=icondata::AiArrowLeftOutlined
            on_click=Callback::new(move |_| {
                navigate_store.with_value(|n| n(href, Default::default()));
            })
        >
            {label}
        </Button>
    };

    match data_testid {
        Some(id) => view! { <div data-testid=id>{button}</div> }.into_any(),
        None => button.into_any(),
    }
}
