use leptos::prelude::*;
use orbital::components::{Text, TextFont};
use orbital::primitives::{MessageBar, MessageBarIntent};

/// Error display for failed runs, using MessageBar with monospace text for error output.
#[component]
pub fn RunErrorDisplay(
    /// Message text to display.
    message: String,
) -> impl IntoView {
    view! {
        <MessageBar intent=MessageBarIntent::Error>
            <Text font=TextFont::Monospace>{message}</Text>
        </MessageBar>
    }
}
