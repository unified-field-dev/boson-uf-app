use leptos::prelude::*;
use orbital::components::{Skeleton, SkeletonItem, SkeletonItemSize};

const REFETCH_SKELETON_ROWS: usize = 5;

/// Skeleton overlay content for DataTable server refetch (search, filter, cancel refresh).
///
/// Use inside `<DataTableLoadingView slot>` as a direct child of [`orbital::primitives::DataTable`].
#[component]
pub fn BosonDataTableRefetchSkeleton() -> impl IntoView {
    view! {
        <Skeleton>
            <div class="orbital-data-table__overlay-skeleton">
                {(0..REFETCH_SKELETON_ROWS)
                    .map(|_| {
                        view! {
                            <SkeletonItem size=Signal::from(SkeletonItemSize::S16) />
                        }
                    })
                    .collect_view()}
            </div>
        </Skeleton>
    }
}
