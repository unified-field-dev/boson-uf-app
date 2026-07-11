use leptos::prelude::*;
use leptos_router::hooks::use_navigate;
use orbital::components::{
    Body1, Caption1, Card, EmptyState, Skeleton, SkeletonItem, SkeletonItemSize, SpacingSize,
    Subtitle2,
};
use orbital::primitives::*;

use crate::components::{
    boson_table_link_styles, BosonCardContent, BosonHelpColumnHeader, BosonTruncatedTableCellLink,
};
use crate::components::{success_rate_help, tasks_overview_help};
use crate::server::TaskSummary;

/// Skeleton rows for the tasks overview table while loading.
#[component]
fn RecentTasksTableSkeleton() -> impl IntoView {
    let skeleton_size = Signal::from(SkeletonItemSize::S16);
    let (style_sheet, class_names) = turf::inline_style_sheet_values! {
        .Table { width: 100%; }
    };

    view! {
        <style>{style_sheet}</style>
        <Card>
            <Table class=class_names.table>
                <TableHeader>
                    <TableRow>
                        <TableHeaderCell><Caption1>"Task"</Caption1></TableHeaderCell>
                        <TableHeaderCell><Caption1>"Queued"</Caption1></TableHeaderCell>
                        <TableHeaderCell><Caption1>"Runs"</Caption1></TableHeaderCell>
                        <TableHeaderCell><Caption1>"Success Rate"</Caption1></TableHeaderCell>
                    </TableRow>
                </TableHeader>
                <TableBody>
                    <Skeleton>
                        {(0..5).map(|_| view! {
                            <TableRow>
                                <TableCell>
                                    <TableCellLayout>
                                        <SkeletonItem size=skeleton_size />
                                    </TableCellLayout>
                                </TableCell>
                                <TableCell>
                                    <TableCellLayout>
                                        <SkeletonItem size=skeleton_size />
                                    </TableCellLayout>
                                </TableCell>
                                <TableCell>
                                    <TableCellLayout>
                                        <SkeletonItem size=skeleton_size />
                                    </TableCellLayout>
                                </TableCell>
                                <TableCell>
                                    <TableCellLayout>
                                        <SkeletonItem size=skeleton_size />
                                    </TableCellLayout>
                                </TableCell>
                            </TableRow>
                        }).collect_view()}
                    </Skeleton>
                </TableBody>
            </Table>
        </Card>
    }
}

/// Table showing the top tasks from the index with navigation.
#[component]
pub fn RecentTasksTable(
    tasks_res: Resource<Result<Vec<TaskSummary>, ServerFnError>>,
) -> impl IntoView {
    let navigate = use_navigate();
    let nav_store = StoredValue::new(navigate);

    let (row_style_sheet, row_classes) = boson_table_link_styles();
    let row_class = StoredValue::new(row_classes.row);
    let (style_sheet, class_names) = turf::inline_style_sheet_values! {
        .Table { width: 100%; }
        .TaskColumn { width: 42%; }
    };

    let top_tasks = Memo::new(move |_| {
        tasks_res
            .get()
            .and_then(|r| r.ok())
            .map(|t| t.into_iter().take(5).collect::<Vec<_>>())
            .unwrap_or_default()
    });

    view! {
        <style>{row_style_sheet}</style>
        <style>{style_sheet}</style>
        <Flex vertical=true gap=SpacingSize::Size160.flex_gap()>
            <Flex justify=FlexJustify::SpaceBetween align=FlexAlign::Center>
                <InfoLabel>
                    <Subtitle2 block=true>"Tasks Overview"</Subtitle2>
                    <InfoLabelInfo slot>
                        {tasks_overview_help()}
                    </InfoLabelInfo>
                </InfoLabel>
                <div data-testid="dashboard-recent-tasks-view-all">
                    <Link href=crate::paths::TASKS>"View All"</Link>
                </div>
            </Flex>
            <Suspense fallback=move || view! { <RecentTasksTableSkeleton /> }>
                {move || match tasks_res.get() {
                    Some(Ok(_)) => {
                        if top_tasks.get().is_empty() {
                            view! {
                                <Card>
                                    <BosonCardContent>
                                        <EmptyState
                                            message="No tasks registered"
                                            description="Register tasks with #[boson::task] to see them here."
                                        />
                                    </BosonCardContent>
                                </Card>
                            }.into_any()
                        } else {
                            view! {
                                <Card>
                                    <Table class=class_names.table>
                                        <TableHeader>
                                            <TableRow>
                                                <TableHeaderCell class=class_names.task_column><Caption1>"Task"</Caption1></TableHeaderCell>
                                                <TableHeaderCell><Caption1>"Queued"</Caption1></TableHeaderCell>
                                                <TableHeaderCell><Caption1>"Runs"</Caption1></TableHeaderCell>
                                                <TableHeaderCell>
                                                    <BosonHelpColumnHeader
                                                        label="Success Rate"
                                                        info=success_rate_help()
                                                    />
                                                </TableHeaderCell>
                                            </TableRow>
                                        </TableHeader>
                                        <TableBody>
                                            <For
                                                each=move || top_tasks.get()
                                                key=|t| t.name.clone()
                                                let:t
                                            >
                                                {
                                                    let name = t.name.clone();
                                                    let name_for_testid = name.clone();
                                                    let href = crate::paths::task(&name);
                                                    let href_nav = href.clone();
                                                    let nav = nav_store.with_value(|n| n.clone());
                                                    let success_rate = t
                                                        .success_rate_pct
                                                        .map(|r| format!("{:.1}%", r))
                                                        .unwrap_or_else(|| "-".to_string());
                                                    view! {
                                                        <TableRow
                                                            class=row_class.with_value(|c| c.clone())
                                                            on:click=move |_| nav(&href_nav, Default::default())
                                                        >
                                                            <TableCell class=class_names.task_column>
                                                                <BosonTruncatedTableCellLink
                                                                    href=href
                                                                    label=name
                                                                    data_testid=format!("dashboard-recent-task-row-{}", name_for_testid)
                                                                />
                                                            </TableCell>
                                                            <TableCell>
                                                                <TableCellLayout>
                                                                    <Body1>{t.jobs_queued}</Body1>
                                                                </TableCellLayout>
                                                            </TableCell>
                                                            <TableCell>
                                                                <TableCellLayout>
                                                                    <Body1>{t.runs_total}</Body1>
                                                                </TableCellLayout>
                                                            </TableCell>
                                                            <TableCell>
                                                                <TableCellLayout>
                                                                    <Body1>{success_rate}</Body1>
                                                                </TableCellLayout>
                                                            </TableCell>
                                                        </TableRow>
                                                    }
                                                }
                                            </For>
                                        </TableBody>
                                    </Table>
                                </Card>
                            }.into_any()
                        }
                    }
                    Some(Err(e)) => view! {
                        <MessageBar intent=MessageBarIntent::Error>{e.to_string()}</MessageBar>
                    }.into_any(),
                    None => view! { <RecentTasksTableSkeleton /> }.into_any(),
                }}
            </Suspense>
        </Flex>
    }
}
