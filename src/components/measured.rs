use dioxus::prelude::*;

use crate::{counts, stats};

/// The measured numbers for one repository as a row of chips: commits and last
/// push from heron, then lines, tests and clippy lints from `counts.json`.
/// Renders nothing when neither source knows the repository.
#[component]
pub fn Measured(repo_url: String) -> Element {
    let live = use_context::<stats::Live>();
    let chips: Vec<String> = stats::chips_for(live.read().as_ref(), &repo_url)
        .into_iter()
        .flatten()
        .chain(counts::chips_for(&repo_url).into_iter().flatten())
        .collect();
    if chips.is_empty() {
        return rsx! {};
    }
    rsx! {
        div { class: "tag-row measured",
            for (i, chip) in chips.iter().enumerate() {
                span { class: "tag tag-c{i % 7}", "{chip}" }
            }
        }
    }
}
