use dioxus::prelude::*;

use crate::changelog::{RELEASES, UPCOMING};

/// The `major.minor` of a version string, e.g. "1.3.1" -> "1.3".
fn major_minor(version: &str) -> &str {
    version.rsplit_once('.').map_or(version, |(head, _)| head)
}

/// Cairn's release history: a major.minor chip filter over a newest-first
/// list of versions, each with its date and notes. A port of zwipe's
/// `components::Changelog`, so the two read the same. Defaults to the latest
/// release's line; "All" shows everything.
#[component]
pub fn Changelog() -> Element {
    // major.minor keys in display order (newest first), deduped.
    let mut minors: Vec<&'static str> = Vec::new();
    for release in UPCOMING.iter().chain(RELEASES) {
        let key = major_minor(release.version);
        if !minors.contains(&key) {
            minors.push(key);
        }
    }

    // None = "All". Starts on the latest released line, not the upcoming
    // ones.
    let mut selected = use_signal(|| RELEASES.first().map(|r| major_minor(r.version)));
    // In each card's key so a filter change remounts the visible cards,
    // replaying their ease-in.
    let filter_key = selected().unwrap_or("all");
    // Upcoming entries come first with an "Upcoming" badge; the first
    // released entry after them is the "Latest".
    let upcoming_count = UPCOMING.len();

    rsx! {
        div { class: "changelog-filter",
            button {
                class: if selected().is_none() { "chip selected" } else { "chip" },
                onclick: move |_| selected.set(None),
                "All"
            }
            for key in minors {
                button {
                    class: if selected() == Some(key) { "chip selected" } else { "chip" },
                    onclick: move |_| selected.set(Some(key)),
                    "{key}"
                }
            }
        }
        div { class: "changelog-list",
            for (i, release) in UPCOMING.iter().chain(RELEASES).enumerate() {
                if selected().is_none_or(|key| key == major_minor(release.version)) {
                    div { key: "{filter_key}-{release.version}", class: "changelog-card",
                        div { class: "changelog-version-row",
                            h2 { class: "changelog-version", "{release.version}" }
                            if !release.date.is_empty() {
                                span { class: "changelog-date", "{release.date}" }
                            }
                            if i < upcoming_count {
                                span { class: "status-tag status-doing", "Upcoming" }
                            } else if i == upcoming_count {
                                span { class: "status-tag status-done", "Latest" }
                            }
                        }
                        ul { class: "changelog-bullets",
                            for entry in release.entries {
                                li { "{entry}" }
                            }
                        }
                    }
                }
            }
        }
    }
}
