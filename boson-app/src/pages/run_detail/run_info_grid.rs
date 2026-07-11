use leptos::prelude::*;
use orbital::components::{Body1, Caption2, Card};
use orbital::primitives::*;

use crate::components::{attempt_help, duration_help, BosonCardContent, BosonTableLink, RunStatusBadge};
use crate::server::RunSummary;

/// Metadata grid showing run details (ID, job, task, status, timestamps, etc.).
#[component]
pub fn RunInfoGrid(run: RunSummary) -> impl IntoView {
    let job_id = run.job_id.clone();
    let job_href = format!("{}?job={}", crate::paths::RUNS, job_id);

    let (style_sheet, class_names) = turf::inline_style_sheet_values! {
        .Label { color: var(--colorNeutralForeground3); }
    };

    view! {
        <style>{style_sheet}</style>
        <Card>
            <BosonCardContent>
                <Grid config=GridConfig::with_gaps(2, 24, 8)>
                    <GridItem><Caption2 class=class_names.label>"Run ID"</Caption2></GridItem>
                    <GridItem><Body1>{run.run_id.clone()}</Body1></GridItem>

                    <GridItem><Caption2 class=class_names.label>"Job ID"</Caption2></GridItem>
                    <GridItem>
                        <BosonTableLink href=job_href>
                            {job_id}
                        </BosonTableLink>
                    </GridItem>

                    <GridItem><Caption2 class=class_names.label>"Task"</Caption2></GridItem>
                    <GridItem><Body1>{run.task_name.clone()}</Body1></GridItem>

                    <GridItem><Caption2 class=class_names.label>"Status"</Caption2></GridItem>
                    <GridItem><RunStatusBadge status=run.status /></GridItem>

                    <GridItem>
                        <InfoLabel>
                            <Caption2 class=class_names.label>"Attempt"</Caption2>
                            <InfoLabelInfo slot>
                                {attempt_help()}
                            </InfoLabelInfo>
                        </InfoLabel>
                    </GridItem>
                    <GridItem><Body1>{run.attempt}</Body1></GridItem>

                    <GridItem><Caption2 class=class_names.label>"Started"</Caption2></GridItem>
                    <GridItem><Body1>{run.started_at.clone()}</Body1></GridItem>

                    <GridItem><Caption2 class=class_names.label>"Finished"</Caption2></GridItem>
                    <GridItem><Body1>{run.finished_at.clone().unwrap_or_else(|| "-".to_string())}</Body1></GridItem>

                    <GridItem>
                        <InfoLabel>
                            <Caption2 class=class_names.label>"Duration"</Caption2>
                            <InfoLabelInfo slot>
                                {duration_help()}
                            </InfoLabelInfo>
                        </InfoLabel>
                    </GridItem>
                    <GridItem><Body1>{run.duration_ms.map(|ms| format!("{} ms", ms)).unwrap_or_else(|| "-".to_string())}</Body1></GridItem>
                </Grid>
            </BosonCardContent>
        </Card>
    }
}
