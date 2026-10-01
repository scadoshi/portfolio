use dioxus::prelude::*;
use zwipe_components::Chip;

use crate::{components::curve::curve, stats::with_separators};

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

/// One row of steller's `BENCHMARKS.md`: `redis-benchmark` at a client count,
/// requests per second for steller and for Redis 8.10, median of three runs on
/// the same laptop with persistence off.
struct Row {
    clients: u32,
    steller_set: u32,
    redis_set: u32,
    steller_get: u32,
    redis_get: u32,
}

const ROWS: [Row; 9] = [
    Row {
        clients: 1,
        steller_set: 52_743,
        redis_set: 15_380,
        steller_get: 53_763,
        redis_get: 14_997,
    },
    Row {
        clients: 2,
        steller_set: 84_175,
        redis_set: 18_657,
        steller_get: 86_655,
        redis_get: 19_616,
    },
    Row {
        clients: 4,
        steller_set: 112_613,
        redis_set: 35_286,
        steller_get: 104_822,
        redis_get: 37_793,
    },
    Row {
        clients: 8,
        steller_set: 144_509,
        redis_set: 57_471,
        steller_get: 131_234,
        redis_get: 65_274,
    },
    Row {
        clients: 16,
        steller_set: 153_374,
        redis_set: 88_496,
        steller_get: 133_333,
        redis_get: 98_814,
    },
    Row {
        clients: 32,
        steller_set: 147_929,
        redis_set: 132_626,
        steller_get: 136_612,
        redis_get: 140_845,
    },
    Row {
        clients: 64,
        steller_set: 140_449,
        redis_set: 146_628,
        steller_get: 134_771,
        redis_get: 169_205,
    },
    Row {
        clients: 128,
        steller_set: 145_138,
        redis_set: 175_131,
        steller_get: 137_741,
        redis_get: 175_747,
    },
    Row {
        clients: 256,
        steller_set: 151_976,
        redis_set: 176_678,
        steller_get: 145_985,
        redis_get: 185_185,
    },
];

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Command {
    Set,
    Get,
}

impl Command {
    fn label(self) -> &'static str {
        match self {
            Self::Set => "SET",
            Self::Get => "GET",
        }
    }

    fn of(self, row: &Row) -> (u32, u32) {
        match self {
            Self::Set => (row.steller_set, row.redis_set),
            Self::Get => (row.steller_get, row.redis_get),
        }
    }
}

const WIDTH: f64 = 640.0;
const HEIGHT: f64 = 320.0;
const LEFT: f64 = 56.0;
const RIGHT: f64 = 16.0;
const TOP: f64 = 16.0;
const BOTTOM: f64 = 44.0;
/// The top of the y axis, above the largest number in the table.
const Y_MAX: f64 = 200_000.0;

/// Client counts are powers of two, so they sit evenly on a log axis: one slot
/// per row.
fn x_at(index: usize) -> f64 {
    let slots = f64::from(u8::try_from(ROWS.len() - 1).unwrap_or(u8::MAX));
    let i = f64::from(u8::try_from(index).unwrap_or(u8::MAX));
    LEFT + (WIDTH - LEFT - RIGHT) * i / slots
}

fn y_at(requests: u32) -> f64 {
    let span = HEIGHT - TOP - BOTTOM;
    HEIGHT - BOTTOM - span * f64::from(requests) / Y_MAX
}

/// steller against Redis 8 on `redis-benchmark`, throughput by client count,
/// from the table in steller's `BENCHMARKS.md`. Inline so the lines take the
/// theme's colors: steller in the primary accent, Redis in the secondary.
#[component]
pub fn Benchmark() -> Element {
    let mut command = use_signal(|| Command::Set);
    let mut tip: Signal<Option<Tip>> = use_signal(|| None);
    let chosen = command();
    let steller: Vec<(f64, f64)> = ROWS
        .iter()
        .enumerate()
        .map(|(i, row)| (x_at(i), y_at(chosen.of(row).0)))
        .collect();
    let redis: Vec<(f64, f64)> = ROWS
        .iter()
        .enumerate()
        .map(|(i, row)| (x_at(i), y_at(chosen.of(row).1)))
        .collect();
    let ticks = [0u32, 50_000, 100_000, 150_000, 200_000];

    rsx! {
        div { class: "bench",
            div { class: "bench-commands",
                for option in [Command::Set, Command::Get] {
                    Chip {
                        key: "{option.label()}",
                        selected: option == chosen,
                        onclick: move |_| command.set(option),
                        "{option.label()}"
                    }
                }
            }
            // The chip is placed by percentages of the chart, so it lives in a
            // wrapper that holds only the chart.
            div { class: "chart-plot", onmouseleave: move |_| tip.set(None),
            svg {
                class: "bench-chart",
                view_box: "0 0 {WIDTH} {HEIGHT}",
                role: "img",
                "aria-label": "{chosen.label()} throughput by client count, steller against Redis 8, one to 256 clients",
                // Grid and axes.
                for tick in ticks {
                    g { key: "y{tick}",
                        line {
                            class: "bench-grid",
                            x1: "{LEFT}", y1: "{y_at(tick)}", x2: "{WIDTH - RIGHT}", y2: "{y_at(tick)}",
                        }
                        text {
                            class: "bench-tick",
                            x: "{LEFT - 8.0}", y: "{y_at(tick) + 4.0}", text_anchor: "end",
                            "{tick / 1000}k"
                        }
                    }
                }
                for (i, row) in ROWS.iter().enumerate() {
                    g { key: "x{row.clients}",
                        line {
                            class: "bench-grid",
                            x1: "{x_at(i)}", y1: "{TOP}", x2: "{x_at(i)}", y2: "{HEIGHT - BOTTOM}",
                        }
                        text {
                            class: "bench-tick",
                            x: "{x_at(i)}", y: "{HEIGHT - BOTTOM + 16.0}", text_anchor: "middle",
                            "{row.clients}"
                        }
                    }
                }
                text {
                    class: "bench-axis",
                    x: "{LEFT + (WIDTH - LEFT - RIGHT) / 2.0}", y: "{HEIGHT - 6.0}", text_anchor: "middle",
                    "concurrent clients, log scale"
                }
                text {
                    class: "bench-axis",
                    transform: "rotate(-90)",
                    x: "{-(TOP + (HEIGHT - TOP - BOTTOM) / 2.0)}", y: "12", text_anchor: "middle",
                    "requests per second"
                }
                // The two series, with a dot on every measured point.
                path { class: "bench-line bench-steller", d: "{curve(&steller)}" }
                path { class: "bench-line bench-redis", d: "{curve(&redis)}" }
                for (i, (x, y)) in steller.iter().enumerate() {
                    circle { key: "s{i}", class: "bench-dot bench-steller", cx: "{x}", cy: "{y}", r: "3" }
                }
                for (i, (x, y)) in redis.iter().enumerate() {
                    circle { key: "r{i}", class: "bench-dot bench-redis", cx: "{x}", cy: "{y}", r: "3" }
                }
                // A hit column per client count: the chip names both servers'
                // numbers and sits above the higher of the two points.
                for (i, row) in ROWS.iter().enumerate() {
                    {
                        let (ours, theirs) = chosen.of(row);
                        let text = format!(
                            "{} clients: steller {}, Redis {}",
                            row.clients,
                            with_separators(u64::from(ours)),
                            with_separators(u64::from(theirs))
                        );
                        let enter = text.clone();
                        let tap = text;
                        let x = x_at(i);
                        let y = y_at(ours.max(theirs));
                        let (left, top) = (x / WIDTH * 100.0, y / HEIGHT * 100.0);
                        let slot = (WIDTH - LEFT - RIGHT) / 8.0;
                        rsx! {
                            rect {
                                key: "h{row.clients}",
                                class: "bench-hit",
                                x: "{x - slot / 2.0}",
                                y: "{TOP}",
                                width: "{slot}",
                                height: "{HEIGHT - TOP - BOTTOM}",
                                onmouseenter: move |_| show(tip, enter.clone(), left, top),
                                onclick: move |_| show(tip, tap.clone(), left, top),
                            }
                        }
                    }
                }
            }
            if let Some(tip) = tip() {
                span {
                    class: "tag tag-c0 bench-tip {anchor(tip.left)}",
                    style: "left: {tip.left}%; top: {tip.top}%;",
                    "{tip.text}"
                }
            }
            }
            div { class: "bench-legend",
                span { class: "bench-key bench-steller", "steller" }
                span { class: "bench-key bench-redis", "Redis 8.10" }
            }
            p { class: "bench-caption",
                "redis-benchmark, median of three runs, same laptop, Redis with persistence off, single-threaded client. The low end is wake-up latency, not processing: pipelined, steller does 509k SET per second on one connection. Above 32 clients every session queues on one mutex. "
                a { href: "https://github.com/scadoshi/steller/blob/main/BENCHMARKS.md", "BENCHMARKS.md" }
                " has the latencies and the caveats."
            }
        }
    }
}
