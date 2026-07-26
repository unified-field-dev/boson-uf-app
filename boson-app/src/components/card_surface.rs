use leptos::prelude::*;
use orbital::components::SpacingSize;
use orbital::primitives::Flex;
use turf::inline_style_sheet_values;

/// Padded card body using Orbital layout primitives.
///
/// Applies uniform inset via [`Flex`] `padding` (inline styles) so card body spacing
/// is not overridden by Orbital `CardContent` stylesheet order. DataTable pages combine
/// this with flex fill classes from [`boson_table_page_layout`].
#[component]
pub fn BosonCardContent(
    /// Additional CSS class(es) to apply.
    #[prop(optional, into)]
    class: MaybeProp<String>,
    /// Child content rendered inside the component.
    children: Children,
) -> impl IntoView {
    view! {
        <Flex
            vertical=true
            full_width=true
            padding=SpacingSize::Size160.inset()
            class=class
        >
            {children()}
        </Flex>
    }
}

/// Layout classes for list pages whose `DataTable` should fill the viewport.
pub struct BosonTablePageClasses {
    pub page: String,
    pub body: String,
    pub card: String,
    pub card_content: String,
}

/// Flex column layout so `DataTables` can fill remaining viewport below the app chrome.
pub fn boson_table_page_layout() -> (&'static str, BosonTablePageClasses) {
    let (style_sheet, class_names) = inline_style_sheet_values! {
        .FillPage {
            display: flex;
            flex-direction: column;
            min-height: calc(100vh - 11rem);
            min-height: calc(100dvh - 11rem);
        }
        .FillBody {
            flex: 1;
            min-height: 0;
            min-width: 0;
            display: flex;
            flex-direction: column;
        }
        .FillCard {
            flex: 1;
            min-height: 0;
            min-width: 0;
            display: flex;
            flex-direction: column;
            overflow: hidden;
        }
        .FillCardContent {
            flex: 1;
            min-height: 0;
            min-width: 0;
            display: flex;
            flex-direction: column;
        }
    };
    (
        style_sheet,
        BosonTablePageClasses {
            page: class_names.fill_page.to_string(),
            body: class_names.fill_body.to_string(),
            card: class_names.fill_card.to_string(),
            card_content: class_names.fill_card_content.to_string(),
        },
    )
}

/// Horizontal scroll + flex fill wrapper for DataTable pages inside [`BosonCardContent`].
///
/// List view disables DataTable's internal horizontal scroll; this shell keeps wide
/// list cards and table columns reachable without clipping the card padding.
#[component]
pub fn BosonDataTableShell(
    /// Data testid.
    #[prop(optional, into)]
    data_testid: MaybeProp<String>,
    /// Child content rendered inside the component.
    children: Children,
) -> impl IntoView {
    let (style_sheet, class_names) = inline_style_sheet_values! {
        .Shell {
            width: 100%;
            min-width: 0;
            max-width: 100%;
            flex: 1;
            min-height: 0;
            display: flex;
            flex-direction: column;
        }
        // List view disables DataTable horizontal scroll; fields grid min-width overflows
        // inside 100%-width cards. Stack/wrap fields so content stays within card padding.
        .Shell .orbital-data-table__list-card-fields {
            grid-template-columns: repeat(2, minmax(0, 1fr));
        }
        .Shell .orbital-data-table__list-card-value {
            overflow-wrap: anywhere;
            word-break: break-word;
        }
        // Table layout: allow the grid body to scroll horizontally inside the card.
        .Shell .orbital-data-table__scroll-host {
            overflow-x: auto;
        }
    };

    view! {
        <style>{style_sheet}</style>
        <div class=class_names.shell data-testid=move || data_testid.get()>
            {children()}
        </div>
    }
}
