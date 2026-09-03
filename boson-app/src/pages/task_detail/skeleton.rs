use leptos::prelude::*;
use orbital::components::{Card, Skeleton, SkeletonItem, SpacingSize};
use orbital::primitives::Flex;

use crate::components::BosonCardContent;

/// Skeleton placeholder while task detail loads.
#[component]
pub fn TaskDetailSkeleton() -> impl IntoView {
    view! {
        <Card>
            <BosonCardContent>
                <Flex vertical=true gap=SpacingSize::Size80.flex_gap()>
                    <Skeleton>
                        <SkeletonItem width="100%".to_string() height="16px".to_string() />
                    </Skeleton>
                    <Skeleton>
                        <SkeletonItem width="60%".to_string() height="16px".to_string() />
                    </Skeleton>
                    <Skeleton>
                        <SkeletonItem width="60%".to_string() height="16px".to_string() />
                    </Skeleton>
                    <Skeleton>
                        <SkeletonItem width="60%".to_string() height="16px".to_string() />
                    </Skeleton>
                    <Skeleton>
                        <SkeletonItem width="40%".to_string() height="32px".to_string() />
                    </Skeleton>
                </Flex>
            </BosonCardContent>
        </Card>
    }
}
