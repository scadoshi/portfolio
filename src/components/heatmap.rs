use dioxus::prelude::*;
use std::collections::BTreeMap;
use zwipe_components::Replay;

use crate::{
    components::{
        chart::{Tip, anchor, month_name, show},
        commits::{ceiling, ticks},
        curve::{area, curve},
    },
    stats::{self, Day, with_separators},
};

/// Cell size and the gap between cells, in SVG units.
const CELL: f64 = 11.0;
const GAP: f64 = 2.0;
const STEP: f64 = CELL + GAP;
/// Room on the left for the weekday labels and the line's ticks.
const LEFT: f64 = 34.0;
/// The month line sits above the grid, on the same columns.
const LINE_TOP: f64 = 10.0;
const LINE_HEIGHT: f64 = 90.0;
const LINE_BOTTOM: f64 = LINE_TOP + LINE_HEIGHT;
/// Where the grid starts, under the line and the month labels.
const TOP: f64 = LINE_BOTTOM + 30.0;
/// A day at or past this many times the median busy day is a peak and gets
/// the lit edge; at most `MAX_PEAKS` of them, the biggest.
const PEAK_RATIO: u32 = 4;
const MAX_PEAKS: usize = 12;

/// The entrance: the line draws first, the dots pop in behind it from
/// `DOTS_AFTER_MS` with `DOT_STAGGER_MS` between them, and the grid sweeps in
/// a column every `SWEEP_STEP_MS`. The durations are in the stylesheet.
const DOTS_AFTER_MS: usize = 250;
const DOT_STAGGER_MS: usize = 70;
const SWEEP_STEP_MS: usize = 12;

/// A first month with fewer days than this in the year is left off the line,
/// so a year that starts mid-month does not open on a dip. The last month
/// stays, partial as it is, and its hover says so.
const FULL_MONTH: usize = 28;

/// Days since 1970-01-01 for a `YYYY-MM-DD` date, or `None` when it does not
/// parse. Howard Hinnant's days-from-civil, which needs no calendar crate.
fn days_from_civil(date: &str) -> Option<i64> {
    let mut parts = date.split('-').map(|part| part.parse::<i64>().ok());
    let (y, m, d) = (parts.next()??, parts.next()??, parts.next()??);
    if !(1..=12).contains(&m) || !(1..=31).contains(&d) {
        return None;
    }
    let y = if m <= 2 { y - 1 } else { y };
    let era = if y >= 0 { y } else { y - 399 } / 400;
    let yoe = y - era * 400;
    let mp = (m + 9) % 12;
    let doy = (153 * mp + 2) / 5 + d - 1;
    let doe = yoe * 365 + yoe / 4 - yoe / 100 + doy;
    Some(era * 146_097 + doe - 719_468)
}

/// 0 for Sunday through 6 for Saturday. 1970-01-01 was a Thursday.
fn weekday(days: i64) -> i64 {
    (days + 4).rem_euclid(7)
}

/// One cell of the grid.
struct Cell {
    column: usize,
    row: usize,
    level: u8,
    date: String,
    count: u32,
}

/// Columns a month label needs before the next one, so two never touch.
const LABEL_SPAN: usize = 3;

/// A month on the line: the columns it spans in the grid and its total.
#[derive(Debug, Clone, PartialEq, Eq)]
struct Series {
    /// `Oct 2025`.
    label: String,
    first_column: usize,
    last_column: usize,
    total: u32,
    days: usize,
}

/// A month label: its column, its name, and the month's contributions.
struct Month {
    column: usize,
    /// `YYYY-MM`, which tells the two Septembers of a year apart.
    key: String,
    name: &'static str,
    total: u32,
}

/// Where each day sits: columns are weeks starting on Sunday, rows are
/// weekdays, like GitHub's own grid. A month label marks the first column
/// whose first day falls in a new month, unless the next month starts within
/// `LABEL_SPAN` columns, which is how a year's partial first month goes unlabeled.
/// Every day's count goes to its month's total, labeled or not, so a label's
/// total is the whole month as far as the year reaches. The series is each
/// month for the line, oldest first, without a partial first month.
fn layout(days: &[Day]) -> (Vec<Cell>, Vec<Month>, Vec<Series>) {
    let Some(first) = days.first().and_then(|day| days_from_civil(&day.date)) else {
        return (Vec::new(), Vec::new(), Vec::new());
    };
    let first_sunday = first - weekday(first);
    let mut cells = Vec::with_capacity(days.len());
    let mut months: Vec<Month> = Vec::new();
    let mut by_month: BTreeMap<&str, Series> = BTreeMap::new();
    let mut last_month = None;
    for day in days {
        let Some(serial) = days_from_civil(&day.date) else {
            continue;
        };
        let column = usize::try_from((serial - first_sunday) / 7).unwrap_or(0);
        let row = usize::try_from(weekday(serial)).unwrap_or(0);
        let key = day.date.get(..7).unwrap_or("");
        let month = day.date.get(5..7).unwrap_or("");
        let series = by_month.entry(key).or_insert_with(|| Series {
            label: format!(
                "{} {}",
                month_name(month).unwrap_or(""),
                day.date.get(..4).unwrap_or("")
            ),
            first_column: column,
            last_column: column,
            total: 0,
            days: 0,
        });
        series.last_column = column;
        series.total = series.total.saturating_add(day.count);
        series.days += 1;
        if row == 0 && Some(month) != last_month {
            if let Some(name) = month_name(month) {
                if months
                    .last()
                    .is_some_and(|previous| column < previous.column + LABEL_SPAN)
                {
                    months.pop();
                }
                months.push(Month {
                    column,
                    key: key.to_string(),
                    name,
                    total: 0,
                });
            }
            last_month = Some(month);
        }
        cells.push(Cell {
            column,
            row,
            level: day.level.min(4),
            date: day.date.clone(),
            count: day.count,
        });
    }
    for month in &mut months {
        month.total = by_month
            .get(month.key.as_str())
            .map_or(0, |series| series.total);
    }
    let mut series: Vec<Series> = by_month.into_values().collect();
    if series.first().is_some_and(|month| month.days < FULL_MONTH) {
        series.remove(0);
    }
    (cells, months, series)
}

/// The dates of the year's outlier days: at or past `PEAK_RATIO` times the
/// median of the days with any contributions, the largest `MAX_PEAKS` of them.
/// A steady busy stretch never qualifies; a spike does.
fn peaks(days: &[Day]) -> Vec<&str> {
    let mut busy: Vec<u32> = days
        .iter()
        .map(|day| day.count)
        .filter(|&n| n > 0)
        .collect();
    if busy.is_empty() {
        return Vec::new();
    }
    busy.sort_unstable();
    let median = busy[busy.len() / 2];
    let floor = median.saturating_mul(PEAK_RATIO);
    let mut peaks: Vec<&Day> = days.iter().filter(|day| day.count >= floor).collect();
    peaks.sort_by_key(|day| std::cmp::Reverse(day.count));
    peaks.truncate(MAX_PEAKS);
    peaks.into_iter().map(|day| day.date.as_str()).collect()
}

/// Where each month lands on the line: x in the middle of its columns, y
/// against `top`, with the line's bottom as the baseline.
fn line_points(series: &[Series], top: u32) -> Vec<(f64, f64)> {
    series
        .iter()
        .map(|month| {
            let middle = px(month.first_column + month.last_column) / 2.0;
            let x = LEFT + middle * STEP + CELL / 2.0;
            let y = LINE_BOTTOM - LINE_HEIGHT * f64::from(month.total) / f64::from(top.max(1));
            (x, y)
        })
        .collect()
}

/// A count as a coordinate. Weeks and weekdays are tiny, nothing is lost.
#[allow(clippy::cast_precision_loss)]
fn px(n: usize) -> f64 {
    n as f64
}

/// The last year of contributions on GitHub as the profile's grid, drawn from
/// the same snapshot the numbers above it use. Renders nothing without one.
/// Hovering (or tapping) a cell or a month label shows its count in a chip.
#[component]
pub fn Heatmap() -> Element {
    let live = use_context::<stats::Live>();
    let mut tip: Signal<Option<Tip>> = use_signal(|| None);
    crate::components::scroll::use_scroll_to_end();
    let live = live.read();
    let Some((calendar, source)) = stats::calendar(live.as_ref()) else {
        return rsx! {};
    };
    let (cells, months, series) = layout(&calendar.days);
    let peaks = peaks(&calendar.days);
    let replay = use_context::<Replay>().0;
    let run = replay();
    let Some(columns) = cells.iter().map(|cell| cell.column + 1).max() else {
        return rsx! {};
    };
    let width = LEFT + px(columns) * STEP;
    let height = TOP + 7.0 * STEP;
    let top = ceiling(series.iter().map(|month| month.total).max().unwrap_or(0));
    let ticks = ticks(top);
    let line_points = line_points(&series, top);
    let (line, area) = (curve(&line_points), area(&line_points, LINE_BOTTOM));
    let last = series.len().saturating_sub(1);
    // A chip anchored at an SVG point, as percentages of the grid.
    let tip_at = move |text: String, x: f64, y: f64| {
        show(tip, text, x / width * 100.0, y / height * 100.0);
    };
    let caption = match source {
        stats::Source::Live => format!(
            "{} contributions on GitHub in the last year, every repository counted: by month above, by day below",
            with_separators(u64::from(calendar.total))
        ),
        stats::Source::AsOf(day) => format!(
            "{} contributions on GitHub in the year to {day}, every repository counted: by month above, by day below",
            with_separators(u64::from(calendar.total))
        ),
    };

    rsx! {
        div { class: "heatmap",
            // The chip is placed by percentages of the grid, so it lives in a
            // wrapper that holds only the grid.
            div { class: "chart-scroll scroll-end",
            div { class: "chart-plot", onmouseleave: move |_| tip.set(None),
            // Keyed on the replay count: a new key is a new SVG, and every
            // animation below starts over.
            for run in [run] {
            svg {
                key: "run{run}",
                class: "heatmap-grid",
                view_box: "0 0 {width} {height}",
                role: "img",
                "aria-label": "{caption}",
                // The month line, on the grid's own columns so a peak sits
                // over the weeks that made it.
                for tick in ticks.iter().copied() {
                    {
                        let y = LINE_BOTTOM - LINE_HEIGHT * f64::from(tick) / f64::from(top.max(1));
                        rsx! {
                            g { key: "t{tick}",
                                line { class: "commits-grid", x1: "{LEFT}", y1: "{y}", x2: "{width}", y2: "{y}" }
                                text { class: "heatmap-label", x: "{LEFT - 5.0}", y: "{y + 3.0}", text_anchor: "end", "{with_separators(u64::from(tick))}" }
                            }
                        }
                    }
                }
                path { class: "commits-area", d: "{area}" }
                // pathLength 1 so one dash animation draws any curve.
                path { class: "commits-line", d: "{line}", path_length: "1" }
                for (i, ((x, y), month)) in line_points.iter().zip(&series).enumerate() {
                    {
                        let text = if i == last {
                            format!("{} in {} so far", with_separators(u64::from(month.total)), month.label)
                        } else {
                            format!("{} in {}", with_separators(u64::from(month.total)), month.label)
                        };
                        let enter = text.clone();
                        let tap = text;
                        let (cx, cy) = (*x, *y);
                        let hit_x = LEFT + px(month.first_column) * STEP;
                        let hit_width = px(month.last_column - month.first_column + 1) * STEP;
                        rsx! {
                            g { key: "s{month.label}",
                                rect {
                                    class: "commits-hit",
                                    x: "{hit_x}",
                                    y: "{LINE_TOP}",
                                    width: "{hit_width}",
                                    height: "{LINE_HEIGHT}",
                                    onmouseenter: move |_| tip_at(enter.clone(), cx, cy),
                                    onclick: move |_| tip_at(tap.clone(), cx, cy),
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
                for month in months.iter() {
                    {
                        let x = LEFT + px(month.column) * STEP;
                        let text = format!("{} in {}", with_separators(u64::from(month.total)), month.name);
                        let enter = text.clone();
                        let tap = text;
                        rsx! {
                            text {
                                key: "m{month.column}",
                                class: "heatmap-label heatmap-month",
                                x: "{x}",
                                y: "{TOP - 5.0}",
                                onmouseenter: move |_| tip_at(enter.clone(), x, TOP - 5.0),
                                onclick: move |_| tip_at(tap.clone(), x, TOP - 5.0),
                                "{month.name}"
                            }
                        }
                    }
                }
                for (row, name) in [(1usize, "Mon"), (3, "Wed"), (5, "Fri")] {
                    text {
                        key: "d{row}",
                        class: "heatmap-label",
                        x: "0",
                        y: "{TOP + px(row) * STEP + CELL - 2.0}",
                        "{name}"
                    }
                }
                for cell in cells.iter() {
                    {
                        let x = LEFT + px(cell.column) * STEP;
                        let y = TOP + px(cell.row) * STEP;
                        let text = format!("{} on {}", cell.count, cell.date);
                        let peak = if peaks.contains(&cell.date.as_str()) { " heat-peak" } else { "" };
                        let enter = text.clone();
                        let tap = text;
                        rsx! {
                            rect {
                                key: "{cell.date}",
                                class: "heatmap-cell heat-{cell.level}{peak}",
                                style: "animation-delay: {cell.column * SWEEP_STEP_MS}ms",
                                x: "{x}",
                                y: "{y}",
                                width: "{CELL}",
                                height: "{CELL}",
                                rx: "2",
                                onmouseenter: move |_| tip_at(enter.clone(), x + CELL / 2.0, y),
                                onclick: move |_| tip_at(tap.clone(), x + CELL / 2.0, y),
                            }
                        }
                    }
                }
            }
            }
            if let Some(tip) = tip() {
                span {
                    class: "tag tag-c0 heatmap-tip {anchor(tip.left)}",
                    style: "left: {tip.left}%; top: {tip.top}%;",
                    "{tip.text}"
                }
            }
            }
            }
            p { class: "heatmap-caption", "{caption}" }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn dates_turn_into_days_and_weekdays() {
        assert_eq!(days_from_civil("1970-01-01"), Some(0));
        assert_eq!(weekday(0), 4, "a Thursday");
        assert_eq!(
            days_from_civil("2026-10-01").map(weekday),
            Some(4),
            "a Thursday"
        );
        assert_eq!(
            days_from_civil("2026-09-27").map(weekday),
            Some(0),
            "a Sunday"
        );
        assert_eq!(days_from_civil("2024-02-29"), Some(19_782));
        assert_eq!(days_from_civil("2026-13-01"), None);
        assert_eq!(days_from_civil("soon"), None);
    }

    #[test]
    fn the_grid_starts_on_the_first_days_sunday_and_marks_months() {
        let day = |date: &str, level| Day {
            date: date.to_string(),
            count: u32::from(level),
            level,
        };
        // 2026-09-30 is a Wednesday, so the first column starts on Sunday the
        // 27th and the Thursday after lands in the same column; Sunday the 4th
        // opens the next one, and October's label goes on it.
        let days = [
            day("2026-09-30", 1),
            day("2026-10-01", 2),
            day("2026-10-04", 3),
            day("2026-10-11", 0),
        ];
        let (cells, months, _) = layout(&days);
        let placed: Vec<(usize, usize, u8)> = cells
            .iter()
            .map(|cell| (cell.column, cell.row, cell.level))
            .collect();
        assert_eq!(placed, [(0, 3, 1), (0, 4, 2), (1, 0, 3), (2, 0, 0)]);
        let labels: Vec<(usize, &str, u32)> = months
            .iter()
            .map(|month| (month.column, month.name, month.total))
            .collect();
        assert_eq!(
            labels,
            [(1, "Oct", 5)],
            "October's two days sum to 3 + 2; September's go unlabeled"
        );
    }

    #[test]
    fn a_partial_first_month_gives_its_label_up_to_the_next() {
        let day = |date: &str| Day {
            date: date.to_string(),
            count: 0,
            level: 0,
        };
        // Sunday 2026-09-27 opens September's only column; October starts the
        // very next column, so "Sep" would sit on top of "Oct".
        let days = [
            day("2026-09-27"),
            day("2026-10-04"),
            day("2026-10-11"),
            day("2026-10-18"),
            day("2026-10-25"),
            day("2026-11-01"),
        ];
        let (_, months, _) = layout(&days);
        let labels: Vec<(usize, &str)> = months
            .iter()
            .map(|month| (month.column, month.name))
            .collect();
        assert_eq!(labels, [(1, "Oct"), (5, "Nov")]);
    }

    #[test]
    fn peaks_are_the_spikes_past_four_times_the_median_busy_day() {
        let day = |date: &str, count| Day {
            date: date.to_string(),
            count,
            level: 0,
        };
        // Busy days 2, 3, 4, 20, 40: the median is 4, the floor 16, so only
        // the 20 and the 40 qualify, biggest first, and the zero day never
        // drags the median down.
        let days = [
            day("2026-09-01", 0),
            day("2026-09-02", 2),
            day("2026-09-03", 3),
            day("2026-09-04", 4),
            day("2026-09-05", 20),
            day("2026-09-06", 40),
        ];
        assert_eq!(peaks(&days), ["2026-09-06", "2026-09-05"]);
        assert_eq!(peaks(&[day("2026-09-01", 0)]), [""; 0]);
    }

    #[test]
    fn a_level_past_four_is_drawn_as_four() {
        let days = [Day {
            date: "2026-10-04".to_string(),
            count: 99,
            level: 9,
        }];
        let (cells, _, _) = layout(&days);
        assert_eq!(cells[0].level, 4);
    }
}
