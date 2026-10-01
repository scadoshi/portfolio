use dioxus::prelude::*;

use crate::stats;

/// The measured numbers for one repository as a row of chips: commits and last
/// push, then lines, tests and clippy lints, all from heron. Renders nothing when
/// the current snapshot does not know the repository.
#[component]
pub fn Measured(repo_url: String) -> Element {
    let live = use_context::<stats::Live>();
    let live = live.read();
    let chips: Vec<String> = stats::chips_for(live.as_ref(), &repo_url)
        .into_iter()
        .flatten()
        .chain(
            stats::count_chips_for(live.as_ref(), &repo_url)
                .into_iter()
                .flatten(),
        )
        .collect();
    if chips.is_empty() {
        return rsx! {};
    }
    rsx! {
        div { class: "tag-row measured",
            // Five chips at most, one accent each: primary, warning,
            // secondary, muted, success.
            for (i, chip) in chips.iter().enumerate() {
                span { class: "tag tag-c{i % 5}", "{chip}" }
            }
        }
    }
}
