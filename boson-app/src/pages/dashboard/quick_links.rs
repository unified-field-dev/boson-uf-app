use leptos::prelude::*;

use crate::components::BosonCardContent;
use orbital::components::{Body1, Card, CardHeader, SpacingSize, Subtitle2};
use orbital::primitives::{Flex, FlexWrap, Link};

/// Quick navigation links to the main Boson pages.
#[component]
pub fn QuickLinks() -> impl IntoView {
    view! {
        <Card>
            <CardHeader>
                <Subtitle2>"Quick Links"</Subtitle2>
            </CardHeader>
            <BosonCardContent>
                <Flex gap=SpacingSize::Size120.flex_gap() wrap=FlexWrap::Wrap>
                    <div id="boson-ql-tasks" data-testid="dashboard-quick-link-tasks">
                        <Link href=crate::paths::TASKS>
                            <Body1>"Tasks"</Body1>
                        </Link>
                    </div>
                    <div id="boson-ql-queue" data-testid="dashboard-quick-link-queue">
                        <Link href=crate::paths::QUEUE>
                            <Body1>"Queue"</Body1>
                        </Link>
                    </div>
                    <div id="boson-ql-runs" data-testid="dashboard-quick-link-runs">
                        <Link href=crate::paths::RUNS>
                            <Body1>"Runs"</Body1>
                        </Link>
                    </div>
                </Flex>
            </BosonCardContent>
        </Card>
    }
}
