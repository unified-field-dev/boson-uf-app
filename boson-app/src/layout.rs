use leptos::prelude::*;
use leptos_router::components::Outlet;
use orbital::components::{
    Navigation, NavigationBody, NavigationConfig, NavigationLink, NavigationMaterial,
};
use uf_integrations::{
    ShellAppBar, ShellLeftNav, UnifiedFieldAppBar, UnifiedFieldShellLayout,
};

use crate::paths;
use crate::AppMetadata;

/// Boson's shell layout: app bar, left navigation, and a router [`Outlet`] for the
/// currently active page.
///
/// Wraps every route declared in [`crate::BosonRoutes`].
#[component]
pub fn BosonLayout() -> impl IntoView {
    let app_name = AppMetadata::name().to_string();
    let selected_value = RwSignal::new(None::<String>);
    let open_categories = RwSignal::new(Vec::<String>::new());

    view! {
        <div data-testid="boson-app-root">
        <UnifiedFieldShellLayout>
            <ShellAppBar slot>
                <UnifiedFieldAppBar
                    app_name=app_name
                    app_id=AppMetadata::id()
                    homepage_url="/".to_string()
                />
            </ShellAppBar>
            <ShellLeftNav slot>
                <Navigation config=NavigationConfig::new().with_selected_value(selected_value).with_open_categories(open_categories)>
                    <NavigationMaterial slot />
                    <NavigationBody slot>
                        <NavigationLink path=paths::ROOT value=paths::ROOT icon=icondata::AiHomeOutlined exact=true test_id="nav-boson-dashboard">"Dashboard"</NavigationLink>
                        <NavigationLink path=paths::TASKS value=paths::TASKS icon=icondata::AiAppstoreOutlined test_id="nav-boson-tasks">"Tasks"</NavigationLink>
                        <NavigationLink path=paths::QUEUE value=paths::QUEUE icon=icondata::AiUnorderedListOutlined test_id="nav-boson-queue">"Queue"</NavigationLink>
                        <NavigationLink path=paths::RUNS value=paths::RUNS icon=icondata::AiHistoryOutlined test_id="nav-boson-runs">"Runs"</NavigationLink>
                    </NavigationBody>
                </Navigation>
            </ShellLeftNav>
            <Outlet />
        </UnifiedFieldShellLayout>
        </div>
    }
}
