use dioxus::prelude::*;
use zwipe_components::Replay;

use crate::{
    components::{
        chart::{Tip, anchor, month_name, show},
        curve::{area, curve},
    },
    stats::{self, WeekCommits, with_separators},
};

const WIDTH: f64 = 640.0;
const HEIGHT: f64 = 220.0;
const LEFT: f64 = 40.0;
const RIGHT: f64 = 12.0;
const TOP: f64 = 14.0;
const BOTTOM: f64 = 28.0;
/// The entrance: the line draws first, the dots pop in behind it from
/// `DOTS_AFTER_MS` with `DOT_STAGGER_MS` between them.
const DOTS_AFTER_MS: usize = 250;
const DOT_STAGGER_MS: usize = 70;

/// A count as a coordinate. Weeks and commit counts are small, nothing is lost.
#[allow(clippy::cast_precision_loss)]
fn px(n: u64) -> f64 {
    n as f64
}

/// The top of a y axis: the largest month, rounded up to a round number so
/// the ticks read cleanly.
pub fn ceiling(max: u32) -> u32 {
    let step = match max {
        0..=20 => 5,
        21..=50 => 10,
        51..=100 => 25,
        101..=250 => 50,
        251..=600 => 100,
        _ => 250,
    };
    max.div_ceil(step).max(1) * step
}

/// The y ticks for a ceiling: the baseline, the top, and the midpoint when it
/// is a round number too. 1,250 gets no 625.
pub fn ticks(top: u32) -> Vec<u32> {
    let mid = top / 2;
    if top <= 100 || mid.is_multiple_of(50) {
        vec![0, mid, top]
    } else {
        vec![0, top]
    }
}

/// One month of commits, the weeks that start in it summed.
#[derive(Debug, Clone, PartialEq, Eq)]
struct Month {
    /// `YYYY-MM`.
    key: String,
    /// `Oct 2025`.
    label: String,
    commits: u32,
}

/// Weeks grouped by the month their Sunday falls in, oldest first. The first
/// and last months are partial, which the hover says for the last one.
fn by_month(weeks: &[WeekCommits]) -> Vec<Month> {
    let mut months: Vec<Month> = Vec::new();
    for week in weeks {
        let Some(key) = week.week.get(..7) else {
            continue;
        };
        match months.last_mut() {
            Some(month) if month.key == key => {
                month.commits = month.commits.saturating_add(week.commits);
            }
            _ => months.push(Month {
                key: key.to_string(),
                label: format!(
                    "{} {}",
                    month_name(week.week.get(5..7).unwrap_or("")).unwrap_or(""),
                    week.week.get(..4).unwrap_or("")
                ),
                commits: week.commits,
            }),
        }
    }
    months
}

/// The drawing area a series is placed in.
#[derive(Clone, Copy)]
struct Frame {
    left: f64,
    right: f64,
    top: f64,
    bottom: f64,
}

/// Where the months land: x evenly across the frame, y against `top`, with
/// `bottom` as the baseline.
fn points(months: &[Month], top: u32, frame: Frame) -> Vec<(f64, f64)> {
    let last = px(u64::try_from(months.len().saturating_sub(1)).unwrap_or(0)).max(1.0);
    let span = frame.bottom - frame.top;
    months
        .iter()
        .enumerate()
        .map(|(i, month)| {
            let x =
                frame.left + (frame.right - frame.left) * px(u64::try_from(i).unwrap_or(0)) / last;
            let y = frame.bottom - span * px(u64::from(month.commits)) / px(u64::from(top.max(1)));
            (x, y)
        })
        .collect()
}

/// The line through `points` and the area under it down to `baseline`.
fn paths(points: &[(f64, f64)], baseline: f64) -> (String, String) {
    (curve(points), area(points, baseline))
}

const TOTAL_FRAME: Frame = Frame {
    left: LEFT,
    right: WIDTH - RIGHT,
    top: TOP,
    bottom: HEIGHT - BOTTOM,
};

/// Commits per month across every repository on the site over the last year,
/// summed from heron's weeks, from the same snapshot as the numbers. Renders
/// nothing without the weeks.
#[component]
pub fn Commits() -> Element {
    let run = use_context::<Replay>().0();
    let live = use_context::<stats::Live>();
    let mut tip: Signal<Option<Tip>> = use_signal(|| None);
    crate::components::scroll::use_scroll_to_end();
    let live = live.read();
    let Some((weeks, source)) = stats::weekly_commits(live.as_ref()) else {
        return rsx! {};
    };
    let months = by_month(weeks);
    let max = months.iter().map(|month| month.commits).max().unwrap_or(0);
    let top = ceiling(max);
    let total_points = points(&months, top, TOTAL_FRAME);
    let (line, area) = paths(&total_points, HEIGHT - BOTTOM);
    let total: u64 = weeks.iter().map(|week| u64::from(week.commits)).sum();
    let ticks = ticks(top);
    let last = months.len().saturating_sub(1);
    let caption = match source {
        stats::Source::Live => format!(
            "{} commits in the last year, only the repositories heron measures",
            with_separators(total)
        ),
        stats::Source::AsOf(day) => format!(
            "{} commits in the year to {day}, only the repositories heron measures",
            with_separators(total)
        ),
    };

    rsx! {
        div { class: "commits",
            // The chip is placed by percentages of the chart, so it lives in a
            // wrapper that holds only the chart.
            div { class: "chart-scroll scroll-end scroll-fade-x",
            div { class: "chart-plot", onmouseleave: move |_| tip.set(None),
            // Keyed on the replay count, as the heatmap is: a theme wipe makes a
            // new key, a new SVG, and every animation below starts over.
            for run in [run] {
            svg {
                key: "run{run}",
                class: "commits-chart",
                view_box: "0 0 {WIDTH} {HEIGHT}",
                role: "img",
                "aria-label": "{caption}",
                for tick in ticks.iter().copied() {
                    {
                        let y = HEIGHT - BOTTOM - (HEIGHT - TOP - BOTTOM) * px(u64::from(tick)) / px(u64::from(top));
                        rsx! {
                            g { key: "t{tick}",
                                line { class: "commits-grid", x1: "{LEFT}", y1: "{y}", x2: "{WIDTH - RIGHT}", y2: "{y}" }
                                text { class: "commits-tick", x: "{LEFT - 6.0}", y: "{y + 4.0}", text_anchor: "end", "{tick}" }
                            }
                        }
                    }
                }
                for ((x, _), month) in total_points.iter().zip(&months) {
                    text {
                        key: "l{month.key}",
                        class: "commits-tick",
                        x: "{x}",
                        y: "{HEIGHT - BOTTOM + 16.0}",
                        text_anchor: "middle",
                        {month.label.get(..3).unwrap_or("")}
                    }
                }
                path { class: "commits-area", d: "{area}" }
                // pathLength 1 so one dash animation draws any curve.
                path { class: "commits-line", d: "{line}", path_length: "1" }
                for (i, ((x, y), month)) in total_points.iter().zip(&months).enumerate() {
                    {
                        let text = if i == last {
                            format!("{} in {} so far", month.commits, month.label)
                        } else {
                            format!("{} in {}", month.commits, month.label)
                        };
                        let enter = text.clone();
                        let tap = text;
                        let (cx, cy) = (*x, *y);
                        let (left, top_pct) = (cx / WIDTH * 100.0, cy / HEIGHT * 100.0);
                        let slot = (WIDTH - LEFT - RIGHT) / px(u64::try_from(last.max(1)).unwrap_or(1));
                        rsx! {
                            g { key: "m{month.key}",
                                // A wide, invisible hit target per month, so the
                                // hover does not need to land on the dot.
                                rect {
                                    class: "commits-hit",
                                    x: "{cx - slot / 2.0}",
                                    y: "{TOP}",
                                    width: "{slot}",
                                    height: "{HEIGHT - TOP - BOTTOM}",
                                    onmouseenter: move |_| show(tip, enter.clone(), left, top_pct),
                                    onclick: move |_| show(tip, tap.clone(), left, top_pct),
                                }
                                circle {
                                    class: "commits-dot",
                                    cx: "{cx}",
                                    cy: "{cy}",
                                    r: "3",
                                    style: "animation-delay: {DOTS_AFTER_MS + i * DOT_STAGGER_MS}ms",
                                }
                            }
                        }
                    }
                }
            }
            }
            if let Some(tip) = tip() {
                span {
                    class: "tag tag-c0 commits-tip {anchor(tip.left)}",
                    style: "left: {tip.left}%; top: {tip.top}%;",
                    "{tip.text}"
                }
            }
            }
            }
            p { class: "commits-caption", "{caption}" }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn the_ceiling_is_a_round_number_at_or_above_the_largest_week() {
        for (max, top) in [
            (0, 5),
            (3, 5),
            (20, 20),
            (21, 30),
            (49, 50),
            (77, 100),
            (101, 150),
            (260, 300),
            (1106, 1250),
        ] {
            assert_eq!(ceiling(max), top, "max {max}");
        }
    }

    #[test]
    fn the_midpoint_tick_only_shows_when_it_is_round() {
        assert_eq!(ticks(20), [0, 10, 20]);
        assert_eq!(ticks(300), [0, 150, 300]);
        assert_eq!(ticks(1250), [0, 1250]);
        assert_eq!(ticks(1500), [0, 750, 1500]);
    }

    #[test]
    fn weeks_are_summed_into_the_month_their_sunday_starts() {
        let week = |date: &str, commits| WeekCommits {
            week: date.to_string(),
            commits,
        };
        let months = by_month(&[
            week("2025-09-28", 1),
            week("2025-10-05", 2),
            week("2025-10-12", 3),
            week("2025-11-02", 4),
        ]);
        let summed: Vec<(&str, &str, u32)> = months
            .iter()
            .map(|month| (month.key.as_str(), month.label.as_str(), month.commits))
            .collect();
        assert_eq!(
            summed,
            [
                ("2025-09", "Sep 2025", 1),
                ("2025-10", "Oct 2025", 5),
                ("2025-11", "Nov 2025", 4)
            ]
        );
    }

    #[test]
    fn months_land_across_the_width_with_the_largest_at_the_top() {
        let month = |key: &str, commits| Month {
            key: key.to_string(),
            label: key.to_string(),
            commits,
        };
        let months = [
            month("2026-07", 0),
            month("2026-08", 10),
            month("2026-09", 20),
        ];
        let points = points(&months, 20, TOTAL_FRAME);
        let close = |a: f64, b: f64| (a - b).abs() < 1e-9;
        assert!(close(points[0].0, LEFT));
        assert!(close(points[2].0, WIDTH - RIGHT));
        assert!(
            close(points[0].1, HEIGHT - BOTTOM),
            "zero sits on the baseline"
        );
        assert!(close(points[2].1, TOP), "the ceiling sits at the top");
        let (line, area) = paths(&points, HEIGHT - BOTTOM);
        assert!(line.starts_with("M 40.0 192.0 C"), "{line}");
        assert!(area.ends_with('Z'), "{area}");
    }
}
