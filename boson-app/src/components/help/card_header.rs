use leptos::prelude::*;
use orbital::components::{Caption1, CardHeader, CardHeaderDescription, Subtitle2};
use orbital::primitives::{InfoLabel, InfoLabelInfo};

/// Card header with optional visible description and deeper info popover.
#[component]
pub fn BosonHelpCardHeader(
    /// Title text.
    title: &'static str,
    /// Description text.
    #[prop(optional)]
    description: Option<&'static str>,
    /// Supplementary info/help content.
    #[prop(optional)]
    info: Option<AnyView>,
) -> impl IntoView {
    let title_row = move || {
        if let Some(info_view) = info {
            view! {
                <InfoLabel>
                    <Subtitle2>{title}</Subtitle2>
                    <InfoLabelInfo slot>
                        {info_view}
                    </InfoLabelInfo>
                </InfoLabel>
            }
            .into_any()
        } else {
            view! { <Subtitle2>{title}</Subtitle2> }.into_any()
        }
    };

    view! {
        {match description {
            Some(desc) => {
                let desc_slot = CardHeaderDescription {
                    children: Box::new(move || {
                        view! { <Caption1>{desc}</Caption1> }.into_any()
                    }),
                };
                view! {
                    <CardHeader card_header_description=desc_slot>
                        {title_row()}
                    </CardHeader>
                }
                .into_any()
            }
            None => view! {
                <CardHeader>
                    {title_row()}
                </CardHeader>
            }
            .into_any(),
        }}
    }
}
