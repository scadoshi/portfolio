use dioxus::prelude::*;
use zwipe_components::Chip;

use crate::stats::{self, with_separators};

/// Which measurement the bars show.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Metric {
    Tests,
    Lines,
    Lints,
}

impl Metric {
    const ALL: [Metric; 3] = [Metric::Tests, Metric::Lines, Metric::Lints];

    fn label(self) -> &'static str {
        match self {
            Self::Tests => "tests",
            Self::Lines => "lines",
            Self::Lints => "clippy lints",
        }
    }

    fn of(self, counts: &stats::Counts) -> Option<u64> {
        match self {
            Self::Tests => Some(counts.tests),
            Self::Lines => Some(counts.lines),
            Self::Lints => counts.clippy_lints,
        }
    }
}

/// A count as a chart coordinate. Counts are far below 2^52, so nothing is lost.
#[allow(clippy::cast_precision_loss)]
fn px(n: u64) -> f64 {
    n as f64
}

const LABEL_WIDTH: f64 = 204.0;
const CHART_WIDTH: f64 = 640.0;
const ROW_HEIGHT: f64 = 22.0;
const BAR_HEIGHT: f64 = 14.0;

/// Every repository heron measures, as bars of one metric, drawn from the same
/// snapshot the cards use. Renders nothing when nothing has been measured.
#[component]
pub fn Fleet() -> Element {
    let live = use_context::<stats::Live>();
    let mut metric = use_signal(|| Metric::Tests);
    let live = live.read();
    let Some((snapshot, source)) = stats::current(live.as_ref()) else {
        return rsx! {};
    };
    let chosen = metric();
    // `(name, value)` for every measured repository, largest first.
    let mut rows: Vec<(&str, u64)> = snapshot
        .repos
        .iter()
        .filter_map(|repo| {
            let value = chosen.of(repo.counts.as_ref()?)?;
            let name = repo.repo.rsplit('/').next().unwrap_or(&repo.repo);
            Some((name, value))
        })
        .collect();
    rows.sort_by(|a, b| b.1.cmp(&a.1).then_with(|| a.0.cmp(b.0)));
    if rows.is_empty() {
        return rsx! {};
    }
    let max = px(rows
        .iter()
        .map(|(_, value)| *value)
        .max()
        .unwrap_or(1)
        .max(1));
    let span = CHART_WIDTH - LABEL_WIDTH - 60.0;
    let height = px(u64::try_from(rows.len()).unwrap_or(u64::MAX)) * ROW_HEIGHT + 4.0;
    let measured_at = snapshot
        .repos
        .iter()
        .filter_map(|repo| repo.counts.as_ref())
        .map(|counts| counts.measured_at.as_str())
        .max()
        .unwrap_or_default();
    let when = match source {
        stats::Source::Live => format!("measured by heron, last at {measured_at}"),
        stats::Source::AsOf(day) => format!("as heron answered on {day}"),
    };

    rsx! {
        div { class: "fleet",
            // The shared Chip, so the toggles look like every other toggle
            // across zwipe.net and the app: selected is accent-secondary.
            div { class: "fleet-metrics",
                for option in Metric::ALL {
                    Chip {
                        key: "{option.label()}",
                        selected: option == chosen,
                        onclick: move |_| metric.set(option),
                        "{option.label()}"
                    }
                }
            }
            div { class: "chart-scroll",
            svg {
                class: "fleet-chart",
                view_box: "0 0 {CHART_WIDTH} {height}",
                role: "img",
                "aria-label": "{chosen.label()} per repository",
                // Each row is a group placed by a transform and each bar's width
                // is a style, so a change of metric glides: rows swap places and
                // bars grow or shrink where they are, with the CSS transitions on
                // .fleet-row and .fleet-bar. Keyed by name so the group survives
                // the re-sort.
                for (i, (name, value)) in rows.iter().enumerate() {
                    {
                        let y = px(u64::try_from(i).unwrap_or(u64::MAX)) * ROW_HEIGHT + 2.0;
                        let width = (px(*value) / max * span).max(1.0);
                        rsx! {
                            g {
                                key: "{name}",
                                class: "fleet-row",
                                style: "transform: translate(0px, {y}px)",
                                text {
                                    class: "fleet-name",
                                    x: "{LABEL_WIDTH - 10.0}",
                                    y: "{BAR_HEIGHT - 3.0}",
                                    text_anchor: "end",
                                    "{name}"
                                }
                                rect {
                                    class: "fleet-bar",
                                    x: "{LABEL_WIDTH}",
                                    y: "0",
                                    height: "{BAR_HEIGHT}",
                                    rx: "3",
                                    style: "width: {width}px",
                                }
                                text {
                                    class: "fleet-value",
                                    x: "0",
                                    y: "{BAR_HEIGHT - 3.0}",
                                    style: "transform: translateX({LABEL_WIDTH + width + 6.0}px)",
                                    "{with_separators(*value)}"
                                }
                            }
                        }
                    }
                }
            }
            }
            div { class: "tag-row fleet-caption",
                span { class: "tag tag-c0", "{when}" }
            }
        }
    }
}
