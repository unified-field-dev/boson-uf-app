use leptos::prelude::*;

use crate::server::RunStatusDto;
use orbital::primitives::{Badge, BadgeAppearance, BadgeColor};

/// Badge component for displaying run status.
#[component]
pub fn RunStatusBadge(
    /// Current status value.
    #[prop(into)]
    status: RunStatusDto,
) -> impl IntoView {
    let (label, appearance, color) = match status {
        RunStatusDto::Running => ("Running", BadgeAppearance::Tint, BadgeColor::Brand),
        RunStatusDto::Success => ("Success", BadgeAppearance::Filled, BadgeColor::Success),
        RunStatusDto::Failed => ("Failed", BadgeAppearance::Filled, BadgeColor::Danger),
        RunStatusDto::Canceled => ("Canceled", BadgeAppearance::Outline, BadgeColor::Warning),
        RunStatusDto::Timeout => ("Timeout", BadgeAppearance::Outline, BadgeColor::Danger),
    };

    view! {
        <Badge appearance=appearance color=color>{label}</Badge>
    }
}
