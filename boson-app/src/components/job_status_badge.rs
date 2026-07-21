use leptos::prelude::*;

use crate::server::JobStatusDto;
use orbital::primitives::{Badge, BadgeAppearance, BadgeColor};

/// Badge component for displaying job status.
#[component]
pub fn JobStatusBadge(
    /// Current status value.
    #[prop(into)] status: JobStatusDto,
) -> impl IntoView {
    let (label, appearance, color) = match status {
        JobStatusDto::Queued => ("Queued", BadgeAppearance::Outline, BadgeColor::Informative),
        JobStatusDto::Running => ("Running", BadgeAppearance::Tint, BadgeColor::Brand),
        JobStatusDto::Success => ("Success", BadgeAppearance::Filled, BadgeColor::Success),
        JobStatusDto::Failed => ("Failed", BadgeAppearance::Filled, BadgeColor::Danger),
        JobStatusDto::Canceled => ("Canceled", BadgeAppearance::Outline, BadgeColor::Warning),
    };

    view! {
        <Badge appearance=appearance color=color>{label}</Badge>
    }
}
