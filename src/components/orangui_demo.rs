//! Orangui's pieces, live on its own project page: a panel of stats, the six
//! palette slots, chips, buttons and a year of days, beside the theme picker's
//! note and a release banner. Every piece is the kit's own, so the demo takes
//! whatever theme the visitor picks.

use dioxus::prelude::*;
use orangui::{
    Banner, BannerStatus, Button, ButtonVariant, Chip, GalleryFooter, HEAT_ROWS, HeatCell,
    HeatGrid, Panel, StatsStrip, ThemeConfig, display_theme_name, heat_span,
};

/// Weeks on the demo's heat grid.
const WEEKS: usize = 52;

/// Peaks lit on the heat grid.
const PEAKS: usize = 3;

/// A made-up year of days, the same on every render so the prerendered page
/// and the browser agree.
fn year_of_days() -> Vec<HeatCell> {
    let scores: Vec<u32> = (0..WEEKS * HEAT_ROWS)
        .map(|i| {
            let row = i % HEAT_ROWS;
            let column = i / HEAT_ROWS;
            // An integer hash of the index, 0..100.
            let mut x = u32::try_from(i).unwrap_or(0).wrapping_mul(0x9E37_79B1) ^ 0x5bd1_e995;
            x ^= x >> 15;
            x = x.wrapping_mul(0x2C1B_3C6D);
            x ^= x >> 12;
            let noise = x % 100;
            // Weekends run quieter, and the year swells toward the autumn.
            let weekday = if row == 0 || row == 6 { 0 } else { 15 };
            let season = u32::try_from(column * 20 / WEEKS).unwrap_or(0);
            (noise + weekday + season).saturating_sub(30)
        })
        .collect();

    let mut ranked: Vec<usize> = (0..scores.len()).collect();
    ranked.sort_by_key(|&i| std::cmp::Reverse(scores[i]));
    let peaks = &ranked[..PEAKS];

    scores
        .iter()
        .enumerate()
        .map(|(i, &score)| HeatCell {
            column: i / HEAT_ROWS,
            row: i % HEAT_ROWS,
            level: match score {
                0..=9 => 0,
                10..=29 => 1,
                30..=49 => 2,
                50..=69 => 3,
                _ => 4,
            },
            peak: peaks.contains(&i),
            key: format!("d{i}"),
        })
        .collect()
}

/// The demo in a panel, with a line under it pointing at the theme picker.
#[component]
pub fn OranguiDemo() -> Element {
    let theme = use_context::<Signal<ThemeConfig>>();
    let theme = theme.read();
    let name = display_theme_name(&theme.name);
    let mode = if theme.is_dark { "Dark" } else { "Light" };
    let width = heat_span(WEEKS);
    let height = heat_span(HEAT_ROWS);

    rsx! {
        figure { class: "project-gallery",
            Panel {
                title: "Running on this page",
                actions: rsx! {
                    GalleryFooter {
                        index: 0,
                        total: 1,
                        caption: "It follows the site's theme. Try the picker up top.".to_string(),
                    }
                },
                div { class: "orangui-demo",
                    Panel {
                        eyebrow: format!("Orangui / {mode}"),
                        status: BannerStatus::Doing,
                        title: name.clone(),
                        StatsStrip {
                            figures: vec![
                                (Some(31), "themes"),
                                (Some(6), "palette slots"),
                                (Some(3), "hosts"),
                            ],
                        }
                        p { class: "demo-label", "Palette slots, cool and warm by turns" }
                        div { class: "demo-swatches",
                            for slot in 1..=6 {
                                span {
                                    class: "demo-swatch",
                                    style: "background: var(--palette-{slot})",
                                    "{slot}"
                                }
                            }
                        }
                        p { class: "demo-label", "Chips" }
                        div { class: "demo-row",
                            Chip { selected: true, onclick: |_| {}, "dioxus" }
                            for label in ["themes", "panel", "nav", "heat grid"] {
                                Chip { key: "{label}", selected: false, onclick: |_| {}, "{label}" }
                            }
                            Chip { selected: false, disabled: true, onclick: |_| {}, "wipe" }
                        }
                        p { class: "demo-label", "Buttons" }
                        div { class: "demo-row",
                            Button { variant: ButtonVariant::Action, onclick: |_| {}, "Save" }
                            Button { variant: ButtonVariant::Action, onclick: |_| {}, "Share" }
                            Button { variant: ButtonVariant::Small, onclick: |_| {}, "Edit" }
                            Button { variant: ButtonVariant::Action, danger: true, onclick: |_| {}, "Delete" }
                            span { class: "status-tag status-done", "Done" }
                            span { class: "status-tag status-doing", "Doing" }
                        }
                        p { class: "demo-label", "A year of days" }
                        div { class: "chart-scroll scroll-fade-x demo-heat",
                            svg {
                                class: "heat-grid",
                                view_box: "0 0 {width} {height}",
                                role: "img",
                                "aria-label": "A made-up year of days, shaded by how busy each was",
                                HeatGrid { cells: year_of_days(), left: 0.0, top: 0.0 }
                            }
                        }
                    }
                    div { class: "demo-side",
                        Panel {
                            eyebrow: "Theme picker",
                            title: "Pick one of 31",
                            p { class: "demo-note",
                                "Every theme ships a dark and a light block. A pick sweeps across the page."
                            }
                            div { class: "demo-row",
                                Chip { selected: true, onclick: |_| {}, "{name}" }
                                Chip { selected: false, onclick: |_| {}, "{mode}" }
                            }
                        }
                        Banner {
                            category: "Release",
                            status: BannerStatus::Done,
                            status_label: "Live",
                            auto_dismiss_secs: 600,
                            "The theme wipe is in: a new theme sweeps across the page instead of blinking."
                        }
                    }
                }
            }
        }
    }
}
