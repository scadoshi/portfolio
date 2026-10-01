use dioxus::prelude::*;
use std::fmt::Write as _;

use crate::stats::{self, WeekCommits, with_separators};

const WIDTH: f64 = 640.0;
const HEIGHT: f64 = 220.0;
const LEFT: f64 = 40.0;
const RIGHT: f64 = 12.0;
const TOP: f64 = 14.0;
const BOTTOM: f64 = 28.0;

/// A count as a coordinate. Weeks and commit counts are small, nothing is lost.
#[allow(clippy::cast_precision_loss)]
fn px(n: u64) -> f64 {
    n as f64
}

/// The top of the y axis: the largest week, rounded up to a round number so
/// the ticks read cleanly.
fn ceiling(max: u32) -> u32 {
    let step = match max {
        0..=20 => 5,
        21..=50 => 10,
        51..=100 => 25,
        101..=250 => 50,
        _ => 100,
    };
    max.div_ceil(step).max(1) * step
}

/// `Jan` for `2026-01-04`.
fn month_of(week: &str) -> &'static str {
    match week.get(5..7) {
        Some("01") => "Jan",
        Some("02") => "Feb",
        Some("03") => "Mar",
        Some("04") => "Apr",
        Some("05") => "May",
        Some("06") => "Jun",
        Some("07") => "Jul",
        Some("08") => "Aug",
        Some("09") => "Sep",
        Some("10") => "Oct",
        Some("11") => "Nov",
        Some("12") => "Dec",
        _ => "",
    }
}

/// What the hover chip says and where it sits, as fractions of the chart.
#[derive(Clone, PartialEq)]
struct Tip {
    text: String,
    left: f64,
    top: f64,
}

fn show(mut tip: Signal<Option<Tip>>, text: String, left: f64, top: f64) {
    tip.set(Some(Tip { text, left, top }));
}

/// Which way the chip hangs off its point: centered in the middle of the
/// chart, and from its edge near either side so it never leaves the panel.
fn anchor(left: f64) -> &'static str {
    if left < 15.0 {
        "tip-start"
    } else if left > 85.0 {
        "tip-end"
    } else {
        ""
    }
}

/// Where the weeks land: x evenly across the chart, y against the ceiling.
fn points(weeks: &[WeekCommits], top: u32) -> Vec<(f64, f64)> {
    let last = px(u64::try_from(weeks.len().saturating_sub(1)).unwrap_or(0)).max(1.0);
    let span = HEIGHT - TOP - BOTTOM;
    weeks
        .iter()
        .enumerate()
        .map(|(i, week)| {
            let x = LEFT + (WIDTH - LEFT - RIGHT) * px(u64::try_from(i).unwrap_or(0)) / last;
            let y = HEIGHT - BOTTOM - span * px(u64::from(week.commits)) / px(u64::from(top));
            (x, y)
        })
        .collect()
}

/// A polyline through `points` as a path, and the same closed down to the
/// baseline for the area under it.
fn paths(points: &[(f64, f64)]) -> (String, String) {
    let mut line = String::new();
    for (i, (x, y)) in points.iter().enumerate() {
        let command = if i == 0 { "M" } else { "L" };
        let _ = write!(line, "{command} {x:.1} {y:.1} ");
    }
    let area = match (points.first(), points.last()) {
        (Some(first), Some(last)) => format!(
            "{line}L {:.1} {:.1} L {:.1} {:.1} Z",
            last.0,
            HEIGHT - BOTTOM,
            first.0,
            HEIGHT - BOTTOM
        ),
        _ => String::new(),
    };
    (line, area)
}

/// Commits per week across every repository on the site over the last year,
/// from the same snapshot as the numbers. Renders nothing without the weeks.
#[component]
pub fn Commits() -> Element {
    let live = use_context::<stats::Live>();
    let mut tip: Signal<Option<Tip>> = use_signal(|| None);
    let live = live.read();
    let Some((weeks, source)) = stats::weekly_commits(live.as_ref()) else {
        return rsx! {};
    };
    let max = weeks.iter().map(|week| week.commits).max().unwrap_or(0);
    let top = ceiling(max);
    let points = points(weeks, top);
    let (line, area) = paths(&points);
    let total: u64 = weeks.iter().map(|week| u64::from(week.commits)).sum();
    let ticks = [0, top / 2, top];
    // A month label on the first week of each month, skipping the first week
    // of the year so the axis starts clean.
    let labels: Vec<(f64, &str)> = points
        .iter()
        .zip(weeks)
        .enumerate()
        .filter(|(i, (_, week))| {
            *i > 0
                && weeks
                    .get(i - 1)
                    .is_some_and(|prev| month_of(&prev.week) != month_of(&week.week))
        })
        .map(|(_, ((x, _), week))| (*x, month_of(&week.week)))
        .collect();
    let caption = match source {
        stats::Source::Live => format!(
            "{} commits across the repositories on this page in the last 52 weeks",
            with_separators(total)
        ),
        stats::Source::AsOf(day) => format!(
            "{} commits across the repositories on this page in the 52 weeks to {day}",
            with_separators(total)
        ),
    };

    rsx! {
        div { class: "commits",
            onmouseleave: move |_| tip.set(None),
            svg {
                class: "commits-chart",
                view_box: "0 0 {WIDTH} {HEIGHT}",
                role: "img",
                "aria-label": "{caption}",
                for tick in ticks {
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
                for (x, name) in labels.iter() {
                    text {
                        key: "l{x}",
                        class: "commits-tick",
                        x: "{x}",
                        y: "{HEIGHT - BOTTOM + 16.0}",
                        text_anchor: "middle",
                        "{name}"
                    }
                }
                path { class: "commits-area", d: "{area}" }
                path { class: "commits-line", d: "{line}" }
                for (i, ((x, y), week)) in points.iter().zip(weeks).enumerate() {
                    {
                        let text = format!("{} in the week of {}", week.commits, week.week);
                        let enter = text.clone();
                        let tap = text;
                        let (cx, cy) = (*x, *y);
                        let (left, top_pct) = (cx / WIDTH * 100.0, cy / HEIGHT * 100.0);
                        rsx! {
                            g { key: "w{i}",
                                // A wide, invisible hit target per week, so the
                                // hover does not need to land on the dot.
                                rect {
                                    class: "commits-hit",
                                    x: "{cx - (WIDTH - LEFT - RIGHT) / 104.0}",
                                    y: "{TOP}",
                                    width: "{(WIDTH - LEFT - RIGHT) / 52.0}",
                                    height: "{HEIGHT - TOP - BOTTOM}",
                                    onmouseenter: move |_| show(tip, enter.clone(), left, top_pct),
                                    onclick: move |_| show(tip, tap.clone(), left, top_pct),
                                }
                                circle { class: "commits-dot", cx: "{cx}", cy: "{cy}", r: "2.5" }
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
        ] {
            assert_eq!(ceiling(max), top, "max {max}");
        }
    }

    #[test]
    fn weeks_land_across_the_width_with_the_largest_at_the_top() {
        let week = |date: &str, commits| WeekCommits {
            week: date.to_string(),
            commits,
        };
        let weeks = [
            week("2026-09-13", 0),
            week("2026-09-20", 10),
            week("2026-09-27", 20),
        ];
        let points = points(&weeks, 20);
        let close = |a: f64, b: f64| (a - b).abs() < 1e-9;
        assert!(close(points[0].0, LEFT));
        assert!(close(points[2].0, WIDTH - RIGHT));
        assert!(
            close(points[0].1, HEIGHT - BOTTOM),
            "zero sits on the baseline"
        );
        assert!(close(points[2].1, TOP), "the ceiling sits at the top");
        let (line, area) = paths(&points);
        assert!(line.starts_with("M 40.0 192.0 L"), "{line}");
        assert!(area.ends_with('Z'), "{area}");
    }

    #[test]
    fn months_come_from_the_week_date() {
        assert_eq!(month_of("2026-01-04"), "Jan");
        assert_eq!(month_of("2026-12-27"), "Dec");
        assert_eq!(month_of("soon"), "");
    }
}
