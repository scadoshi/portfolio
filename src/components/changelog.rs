use dioxus::prelude::*;
use orangui::Chip;

use crate::changelog::{Date, RELEASES, UPCOMING};

/// Cairn's release history: a month chip filter over a newest-first list of
/// versions, each with its date and notes. The cards are a port of zwipe's
/// `components::Changelog`, so the two read the same. Defaults to the latest
/// release's month; "All" shows everything. Upcoming versions have no date
/// and sit with the newest month.
#[component]
pub fn Changelog() -> Element {
    // The newest month, which upcoming versions are filed under.
    let newest = RELEASES.first().and_then(|r| r.date).map(Date::month_start);
    let month_of = move |date: Option<Date>| date.map_or(newest, |d| Some(d.month_start()));

    // Months in display order (newest first), deduped.
    let mut months: Vec<Date> = Vec::new();
    for release in UPCOMING.iter().chain(RELEASES) {
        if let Some(month) = month_of(release.date)
            && !months.contains(&month)
        {
            months.push(month);
        }
    }

    // None = "All".
    let mut selected = use_signal(|| newest);
    // In each card's key so a filter change remounts the visible cards,
    // replaying their ease-in.
    let filter_key = selected().map_or_else(|| "all".to_string(), Date::month_label);
    // Upcoming entries come first with an "Upcoming" badge; the first
    // released entry after them is the "Latest".
    let upcoming_count = UPCOMING.len();

    rsx! {
        div { class: "changelog-filter",
            Chip {
                selected: selected().is_none(),
                onclick: move |_| selected.set(None),
                "All"
            }
            for month in months {
                Chip {
                    key: "{month.month_label()}",
                    selected: selected() == Some(month),
                    onclick: move |_| selected.set(Some(month)),
                    "{month.month_label()}"
                }
            }
        }
        div { class: "changelog-list",
            for (i, release) in UPCOMING.iter().chain(RELEASES).enumerate() {
                if selected().is_none() || selected() == month_of(release.date) {
                    div { key: "{filter_key}-{release.version}", class: "changelog-card",
                        div { class: "changelog-version-row",
                            h2 { class: "changelog-version", "{release.version}" }
                            if let Some(date) = release.date {
                                span { class: "changelog-date", "{date}" }
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
