use leptos::prelude::*;
use orbital::components::{TableCellLayout, TableCellLayoutConfig};
use orbital::primitives::{Link, Tooltip};
use turf::inline_style_sheet_values;

pub struct BosonTableLinkClasses {
    pub row: String,
}

/// Shared hover styles for clickable table rows.
pub fn boson_table_link_styles() -> (&'static str, BosonTableLinkClasses) {
    let (style_sheet, class_names) = inline_style_sheet_values! {
        .Row { cursor: pointer; }
        .Row:hover { background: var(--orb-color-surface-canvas-hover); }
    };
    (
        style_sheet,
        BosonTableLinkClasses {
            row: class_names.row.to_string(),
        },
    )
}

/// Orbital Link for table cells and metadata grids.
#[component]
pub fn BosonTableLink(
    /// Link target.
    href: String,
    /// Child content rendered inside the component.
    children: Children,
) -> impl IntoView {
    view! {
        <Link href=href>
            {children()}
        </Link>
    }
}

/// Truncated table cell link with ellipsis and full-text tooltip on hover.
#[component]
pub fn BosonTruncatedTableCellLink(
    /// Link target.
    href: String,
    /// Label text.
    label: String,
    /// Optional data testid.
    #[prop(optional, into)] data_testid: Option<String>,
) -> impl IntoView {
    view! {
        <TableCellLayout config=TableCellLayoutConfig { truncate: true }>
            <Tooltip content=label.clone()>
                <span data-testid=data_testid>
                    <Link href=href>{label}</Link>
                </span>
            </Tooltip>
        </TableCellLayout>
    }
}
