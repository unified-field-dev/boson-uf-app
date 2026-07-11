use leptos::prelude::*;
use orbital::components::{Card, Skeleton, SkeletonItem, SpacingSize};
use orbital::primitives::Flex;

use crate::components::BosonCardContent;

/// Skeleton placeholder while task detail loads.
#[component]
pub fn TaskDetailSkeleton() -> impl IntoView {
    let (style_sheet, class_names) = turf::inline_style_sheet_values! {
        .Line { width: 100%; height: 16px; }
        .LineShort { width: 60%; height: 16px; }
        .Actions { width: 40%; height: 32px; margin-top: 8px; }
    };

    view! {
        <style>{style_sheet}</style>
        <Card>
            <BosonCardContent>
                <Flex vertical=true gap=SpacingSize::Size80.flex_gap()>
                    <Skeleton><SkeletonItem class=class_names.line /></Skeleton>
                    <Skeleton><SkeletonItem class=class_names.line_short /></Skeleton>
                    <Skeleton><SkeletonItem class=class_names.line_short /></Skeleton>
                    <Skeleton><SkeletonItem class=class_names.line_short /></Skeleton>
                    <Skeleton><SkeletonItem class=class_names.actions /></Skeleton>
                </Flex>
            </BosonCardContent>
        </Card>
    }
}
