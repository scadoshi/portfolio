//! Orangui's pieces, live on its own project page: a panel per component,
//! each with a line on when to use it, the piece running in the site's
//! theme, and a link to its source. The app pieces run in a phone of their
//! own ([`super::orangui_phone`]).

use dioxus::prelude::*;
use gloo_timers::future::TimeoutFuture;
use orangui::{
    APP_CSS, ActionBar, ActionButton, AlertDialog, Banner, BannerStatus, Button, ButtonVariant,
    ChartTip, Chip, DiagramArrow, DiagramDefs, DiagramNode, DiagramTone, DialogAction,
    DialogHostView, GalleryFooter, GalleryFrame, HEAT_CELL, HEAT_ROWS, HeatCell, HeatGrid, HeatHit,
    HintBullet, HintBullets, HintKey, HintLine, InfoButton, NavDropdown, Panel, PasswordInput,
    Replay, StatsStrip, TOAST_CSS, ThemeConfig, area, curve, display_theme_name, heat_span,
    use_dialog_host, use_overlay_back_stack,
};

use super::orangui_phone::OranguiPhone;

/// Where the source links point: orangui's default branch.
const REPO: &str = "https://github.com/scadoshi/orangui/blob/main";

/// The nav's S, the art the Decode panel resolves.
const LOGO_S: &str = include_str!("../../assets/s.txt");

/// Weeks on the heat grid.
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

/// A link to a file in orangui's repository, named by the file. `path` may
/// carry a `#L..` anchor to a stylesheet section.
#[component]
pub(super) fn Source(path: &'static str) -> Element {
    let file = path.split('#').next().unwrap_or(path);
    let name = file.rsplit('/').next().unwrap_or(file);
    rsx! {
        a { class: "panel-action", href: "{REPO}/{path}",
            "{name}"
            span { class: "ext", "\u{2197}" }
        }
    }
}

/// Every piece of the kit in a panel of its own, the phone for the app
/// pieces, and the dialog and banner layers they open into.
#[component]
pub fn OranguiShowcase() -> Element {
    let stack = use_overlay_back_stack();
    use_context_provider(|| stack);
    let host = use_dialog_host();
    use_context_provider(|| host);
    let banners = use_signal(Vec::<u32>::new);

    rsx! {
        // The app sheets, for the dialogs, the hints, the password field and
        // the phone. None of their class names appear elsewhere on the site.
        document::Style { {APP_CSS} }
        document::Style { {TOAST_CSS} }
        Panel { title: "Every piece, live",
            section { class: "project-section kit-intro",
                p {
                    "Each panel below runs one piece of the kit in this page's theme, with a link to its source. Pick another theme up top and they all follow."
                }
                p { class: "kit-credit",
                    "The layout follows the "
                    a { href: "https://dioxuslabs.com/components", "Dioxus component library" }
                    ". The toast stack in the phone builds on "
                    a { href: "https://github.com/DioxusLabs/components", "dioxus-primitives" }
                    "."
                }
            }
        }
        div { class: "kit-grid",
            PanelShowcase {}
            ButtonShowcase {}
            ChipShowcase {}
            BannerShowcase { banners }
            DropdownShowcase {}
            DialogShowcase {}
            PasswordShowcase {}
            HintShowcase {}
            StatsShowcase {}
            PaletteShowcase {}
            DecodeShowcase {}
            GalleryShowcase {}
            ChartShowcase {}
            DiagramShowcase {}
        }
        HeatShowcase {}
        OranguiPhone {}
        div { class: "banner-stack kit-banners",
            for id in banners() {
                Banner {
                    key: "{id}",
                    category: "Release",
                    status: BannerStatus::Done,
                    status_label: "Live",
                    auto_dismiss_secs: 8,
                    "A banner like this one announces a release. It leaves when the bar runs out."
                }
            }
        }
        DialogHostView {}
    }
}

// == the panels ==

#[component]
fn PanelShowcase() -> Element {
    let mut saved = use_signal(|| false);
    rsx! {
        Panel {
            eyebrow: "Eyebrow",
            status: if saved() { BannerStatus::Done } else { BannerStatus::Doing },
            status_label: if saved() { "Saved" } else { "Unsaved" },
            title: "Panel",
            actions: rsx! {
                Source { path: "src/panel.rs" }
                Source { path: "src/action_bar.rs" }
            },
            p { class: "kit-use",
                "The card everything sits in: an eyebrow, a title, a status pill, the body. An ActionBar holds a screen's buttons; the apps pin it to the bottom."
            }
            ActionBar { class: "kit-action-bar".to_string(),
                Button {
                    variant: ButtonVariant::Action,
                    disabled: !saved(),
                    onclick: move |_| saved.set(false),
                    "Undo"
                }
                Button {
                    variant: ButtonVariant::Action,
                    disabled: saved(),
                    onclick: move |_| saved.set(true),
                    "Save"
                }
            }
        }
    }
}

#[component]
fn ButtonShowcase() -> Element {
    let mut pressed = use_signal(|| None::<&'static str>);
    rsx! {
        Panel {
            title: "Button",
            actions: rsx! {
                Source { path: "src/button.rs" }
                Source { path: "assets/components.css#L171-L247" }
            },
            p { class: "kit-use", "One component in three sizes. Pick the size by where the button sits." }
            div { class: "kit-buttons",
                p { class: "kit-label", "Full width, to finish a form" }
                Button { onclick: move |_| pressed.set(Some("Sign in")), "Sign in" }
                div { class: "kit-button-row",
                    div {
                        p { class: "kit-label", "In an action bar" }
                        Button {
                            variant: ButtonVariant::Action,
                            onclick: move |_| pressed.set(Some("Share")),
                            "Share"
                        }
                    }
                    div {
                        p { class: "kit-label", "Can't be undone" }
                        Button {
                            variant: ButtonVariant::Action,
                            danger: true,
                            onclick: move |_| pressed.set(Some("Delete")),
                            "Delete"
                        }
                    }
                    div {
                        p { class: "kit-label", "Inline, beside text" }
                        Button {
                            variant: ButtonVariant::Small,
                            onclick: move |_| pressed.set(Some("Edit")),
                            "Edit"
                        }
                    }
                    div {
                        p { class: "kit-label", "Not ready yet" }
                        Button {
                            variant: ButtonVariant::Action,
                            disabled: true,
                            onclick: |_| {},
                            "Send"
                        }
                    }
                }
            }
            p { class: "kit-note",
                match pressed() {
                    Some(label) => format!("Pressed {label}."),
                    None => "Nothing pressed yet.".to_string(),
                }
            }
        }
    }
}

/// The chips' labels, and whether each starts on.
const CHIPS: [(&str, bool); 4] = [
    ("rust", true),
    ("dioxus", false),
    ("wasm", true),
    ("css", false),
];

#[component]
fn ChipShowcase() -> Element {
    let mut on = use_signal(|| CHIPS.map(|(_, selected)| selected));
    rsx! {
        Panel {
            title: "Chip and tag",
            actions: rsx! {
                Source { path: "src/chip.rs" }
                Source { path: "assets/components.css#L54-L170" }
            },
            p { class: "kit-use",
                "A chip toggles a filter. A tag only labels: a palette tone for decoration, an outcome tone when it reports a result."
            }
            p { class: "kit-label", "Chips, tap to toggle" }
            div { class: "kit-row",
                for (i, (label, _)) in CHIPS.into_iter().enumerate() {
                    Chip {
                        key: "{label}",
                        selected: on.read().get(i).copied().unwrap_or(false),
                        onclick: move |_| {
                            if let Some(chip) = on.write().get_mut(i) {
                                *chip = !*chip;
                            }
                        },
                        "{label}"
                    }
                }
                Chip { selected: false, disabled: true, onclick: |_| {}, "go" }
            }
            p { class: "kit-label", "Palette tones, slots 1 to 6" }
            div { class: "kit-row",
                span { class: "tag tag-p1", "backend" }
                span { class: "tag tag-p2", "frontend" }
                span { class: "tag tag-p3", "cli" }
                span { class: "tag tag-p4", "app" }
                span { class: "tag tag-p5", "docs" }
                span { class: "tag tag-p6", "infra" }
            }
            p { class: "kit-label", "Outcome tones" }
            div { class: "kit-row",
                span { class: "tag tag-success", "passed" }
                span { class: "tag tag-warning", "slow" }
                span { class: "tag tag-error", "failed" }
                span { class: "tag tag-muted", "archived" }
            }
        }
    }
}

#[component]
fn BannerShowcase(banners: Signal<Vec<u32>>) -> Element {
    let mut raised = use_signal(|| 0u32);
    rsx! {
        Panel {
            title: "Banner",
            actions: rsx! {
                Source { path: "src/banner.rs" }
                Source { path: "assets/components.css#L1047-L1238" }
            },
            p { class: "kit-use",
                "News a visitor should see once, like a release. It counts down and leaves on its own, or on its close button."
            }
            div { class: "kit-row",
                Button {
                    variant: ButtonVariant::Action,
                    onclick: move |_| {
                        raised += 1;
                        let id = raised();
                        let mut list = banners.write();
                        list.push(id);
                        // The rest have long since left on their own.
                        if list.len() > 3 {
                            list.remove(0);
                        }
                    },
                    "Raise a banner"
                }
            }
            p { class: "kit-note", "It rises at the bottom left of the window." }
        }
    }
}

/// The dropdown's rows.
const SECTIONS: [&str; 4] = ["Products", "Systems", "Tooling", "Experiments"];

#[component]
fn DropdownShowcase() -> Element {
    let mut open = use_signal(|| false);
    let mut picked = use_signal(|| None::<&'static str>);
    rsx! {
        Panel {
            title: "NavDropdown",
            actions: rsx! {
                Source { path: "src/nav_dropdown.rs" }
                Source { path: "assets/components.css#L820-L1046" }
            },
            p { class: "kit-use",
                "A menu behind a nav pill. The host holds the open state, so a pick can close it."
            }
            div { class: "kit-dropdown",
                NavDropdown { open, label: picked().unwrap_or("Sections").to_string(),
                    for name in SECTIONS {
                        button {
                            key: "{name}",
                            class: if picked() == Some(name) { "nav-dropdown-item active" } else { "nav-dropdown-item" },
                            onclick: move |_| {
                                picked.set(Some(name));
                                open.set(false);
                            },
                            "{name}"
                        }
                    }
                }
            }
        }
    }
}

#[component]
fn DialogShowcase() -> Element {
    let mut confirm_open = use_signal(|| false);
    let mut name_open = use_signal(|| false);
    let mut name = use_signal(String::new);
    let mut outcome = use_signal(|| None::<String>);
    let blank = use_memo(move || name().trim().is_empty());
    let delete = use_callback(move |()| outcome.set(Some("Deleted the draft.".to_string())));
    let save = ActionButton::new("Save")
        .disabled(blank)
        .busy("Saving...", move || async move {
            TimeoutFuture::new(1200).await;
            outcome.set(Some(format!("Saved as {}.", name.peek().trim())));
            true
        });

    rsx! {
        Panel {
            title: "AlertDialog",
            actions: rsx! {
                Source { path: "src/alert_dialog.rs" }
                Source { path: "assets/app.css#L233-L439" }
            },
            p { class: "kit-use",
                "A question that has to be answered before anything else happens. An ActionButton can wait on input, or stay busy while it works."
            }
            div { class: "kit-row",
                Button {
                    variant: ButtonVariant::Action,
                    onclick: move |_| confirm_open.set(true),
                    "Delete the draft"
                }
                Button {
                    variant: ButtonVariant::Action,
                    onclick: move |_| {
                        name.set(String::new());
                        name_open.set(true);
                    },
                    "Name the draft"
                }
            }
            p { class: "kit-note",
                {outcome().unwrap_or_else(|| "Save stays off until you type a name.".to_string())}
            }
            AlertDialog {
                open: confirm_open,
                title: "Delete the draft?",
                actions: vec![
                    DialogAction::Dismiss("Cancel".to_string()),
                    DialogAction::Confirm {
                        label: "Delete".to_string(),
                        danger: true,
                        on_confirm: delete,
                    },
                ],
                "It goes for good. There is no undo."
            }
            AlertDialog {
                open: name_open,
                title: "Name the draft",
                actions: vec![DialogAction::Dismiss("Cancel".to_string()), save.into()],
                input {
                    class: "input kit-dialog-input",
                    placeholder: "A name",
                    value: "{name}",
                    oninput: move |event| name.set(event.value()),
                }
            }
        }
    }
}

#[component]
fn PasswordShowcase() -> Element {
    let value = use_signal(|| "orangutan".to_string());
    rsx! {
        Panel {
            title: "PasswordInput",
            actions: rsx! {
                Source { path: "src/password_input.rs" }
                Source { path: "assets/components.css#L1359-L1390" }
            },
            p { class: "kit-use",
                "Any field that takes a password. Show lets the user check what they typed."
            }
            label { class: "kit-label", r#for: "kit-password", "Password" }
            PasswordInput {
                value,
                id: "kit-password".to_string(),
                class: "input".to_string(),
                placeholder: "Password".to_string(),
            }
        }
    }
}

#[component]
fn HintShowcase() -> Element {
    let mut open = use_signal(|| false);
    rsx! {
        Panel {
            title: "InfoButton and HintKey",
            actions: rsx! {
                Source { path: "src/hint.rs" }
                Source { path: "assets/components.css#L1945-L1985" }
            },
            p { class: "kit-use",
                "Help inside a line of text. The ? opens a hint; a HintKey draws the button the hint talks about."
            }
            p { class: "kit-hint-line",
                "Tap "
                HintKey { "Save" }
                " to keep a theme, or "
                HintKey { color: "--palette-1".to_string(), "Back" }
                " to drop it."
                InfoButton { onclick: move |_| open.set(true) }
            }
            AlertDialog {
                open,
                title: "Picking a theme",
                actions: vec![DialogAction::Dismiss("Got it".to_string())],
                HintLine { "A pick shows at once, before you keep it." }
                HintBullets {
                    HintBullet {
                        HintKey { "Save" }
                        " keeps it."
                    }
                    HintBullet {
                        HintKey { color: "--palette-1".to_string(), "Back" }
                        " puts the old one back."
                    }
                }
            }
        }
    }
}

#[component]
fn StatsShowcase() -> Element {
    // Its own replay counter, so the button reruns these figures and nothing
    // else on the page.
    let mut replay = use_signal(|| 0u32);
    use_context_provider(|| Replay(replay));
    rsx! {
        Panel {
            title: "StatsStrip",
            actions: rsx! {
                Source { path: "src/stats_strip.rs" }
                Source { path: "src/entrance.rs" }
            },
            p { class: "kit-use",
                "Headline numbers. Each rolls up to its value as the page loads."
            }
            StatsStrip {
                figures: vec![(Some(31), "themes"), (Some(6), "palette slots"), (Some(3), "hosts")],
            }
            div { class: "kit-row kit-replay",
                Button { variant: ButtonVariant::Small, onclick: move |_| replay += 1, "Replay" }
            }
        }
    }
}

#[component]
fn PaletteShowcase() -> Element {
    let theme = use_context::<Signal<ThemeConfig>>();
    let theme = theme.read();
    let name = display_theme_name(&theme.name);
    let mode = if theme.is_dark { "dark" } else { "light" };
    rsx! {
        Panel {
            title: "Theme palette",
            actions: rsx! {
                Source { path: "assets/themes.css" }
                Source { path: "src/theme.rs" }
            },
            p { class: "kit-use",
                "Six slots in every theme, for color that means nothing: tags, chart lines, chips. Cool and warm take turns."
            }
            div { class: "kit-swatches",
                for slot in 1..=6 {
                    span {
                        key: "{slot}",
                        class: "kit-swatch",
                        style: "background: var(--palette-{slot})",
                        "{slot}"
                    }
                }
            }
            p { class: "kit-note", "Showing {name}, {mode}." }
        }
    }
}

#[component]
fn DecodeShowcase() -> Element {
    let mut replay = use_signal(|| 0u32);
    use_context_provider(|| Replay(replay));
    let mut hovering = use_signal(|| false);
    rsx! {
        Panel {
            title: "Decode",
            actions: rsx! {
                Source { path: "src/entrance.rs" }
            },
            p { class: "kit-use",
                "Block-glyph art that resolves from noise, as the S in the nav does on load. Hover it and it shimmers."
            }
            div { class: "kit-decode",
                pre {
                    class: "kit-decode-art",
                    "aria-label": "The S logo",
                    onmouseenter: move |_| hovering.set(true),
                    onmouseleave: move |_| hovering.set(false),
                    orangui::Decode { text: LOGO_S, hover: hovering }
                }
                Button { variant: ButtonVariant::Small, onclick: move |_| replay += 1, "Replay" }
            }
        }
    }
}

/// The gallery's slides: a host of the kit and what it is.
const SLIDES: [(&str, &str); 3] = [
    ("zwipe", "The app and zwipe.net"),
    ("portfolio", "This site"),
    ("cairn", "A counter app"),
];

#[component]
fn GalleryShowcase() -> Element {
    let index = use_signal(|| 0usize);
    let total = SLIDES.len();
    let (name, line) = SLIDES.get(index()).copied().unwrap_or(("", ""));
    rsx! {
        Panel {
            title: "GalleryFrame",
            actions: rsx! {
                Source { path: "src/gallery.rs" }
                Source { path: "assets/site.css#L40-L119" }
            },
            p { class: "kit-use",
                "One item at a time, with arrows and a counter. The demo videos on this site and the Zwipe diagrams sit in one."
            }
            GalleryFrame { index, total, noun: "host", class: "kit-slides".to_string(),
                div { key: "{index()}", class: "kit-slide tag tag-p{index() + 1}",
                    span { class: "kit-slide-name", "{name}" }
                    span { "{line}" }
                }
            }
            GalleryFooter {
                index: index(),
                total,
                caption: "The three hosts that run orangui".to_string(),
            }
        }
    }
}

/// Twelve made-up months for the line chart.
const MONTHS: [(&str, u32); 12] = [
    ("Jan", 12),
    ("Feb", 18),
    ("Mar", 15),
    ("Apr", 27),
    ("May", 22),
    ("Jun", 31),
    ("Jul", 29),
    ("Aug", 40),
    ("Sep", 36),
    ("Oct", 48),
    ("Nov", 44),
    ("Dec", 57),
];
const CHART_W: f64 = 480.0;
const CHART_H: f64 = 200.0;
const CHART_LEFT: f64 = 30.0;
const CHART_RIGHT: f64 = 12.0;
const CHART_TOP: f64 = 12.0;
const CHART_BOTTOM: f64 = 26.0;
const CHART_MAX: u32 = 60;

/// A small count as a coordinate.
fn px(n: u32) -> f64 {
    f64::from(n)
}

#[component]
fn ChartShowcase() -> Element {
    let mut tip = use_signal(|| None::<(String, f64, f64)>);
    let last = u32::try_from(MONTHS.len() - 1).unwrap_or(1);
    let span_x = CHART_W - CHART_LEFT - CHART_RIGHT;
    let span_y = CHART_H - CHART_TOP - CHART_BOTTOM;
    let baseline = CHART_H - CHART_BOTTOM;
    let points: Vec<(f64, f64)> = MONTHS
        .iter()
        .zip(0u32..)
        .map(|((_, n), i)| {
            (
                CHART_LEFT + span_x * px(i) / px(last),
                baseline - span_y * px(*n) / px(CHART_MAX),
            )
        })
        .collect();
    let slot = span_x / px(last);
    rsx! {
        Panel {
            title: "ChartTip",
            actions: rsx! {
                Source { path: "src/chart_math.rs" }
                Source { path: "src/chart_tip.rs" }
            },
            p { class: "kit-use",
                "A value over time. The kit's curve and area draw a line that never overshoots a point, and ChartTip names the month under the pointer."
            }
            div { class: "chart-scroll scroll-fade-x",
                div { class: "kit-plot", onmouseleave: move |_| tip.set(None),
                    svg {
                        class: "commits-chart",
                        view_box: "0 0 {CHART_W} {CHART_H}",
                        role: "img",
                        "aria-label": "A made-up year that climbs from 12 a month in January to 57 in December",
                        for tick in [0, CHART_MAX / 2, CHART_MAX] {
                            {
                                let y = baseline - span_y * px(tick) / px(CHART_MAX);
                                rsx! {
                                    g { key: "t{tick}",
                                        line { class: "commits-grid", x1: "{CHART_LEFT}", y1: "{y}", x2: "{CHART_W - CHART_RIGHT}", y2: "{y}" }
                                        text { class: "commits-tick", x: "{CHART_LEFT - 6.0}", y: "{y + 4.0}", text_anchor: "end", "{tick}" }
                                    }
                                }
                            }
                        }
                        for ((x , _) , (month , _)) in points.iter().zip(MONTHS) {
                            text {
                                key: "l{month}",
                                class: "commits-tick",
                                x: "{x}",
                                y: "{baseline + 16.0}",
                                text_anchor: "middle",
                                "{month}"
                            }
                        }
                        path { class: "commits-area", d: "{area(&points, baseline)}" }
                        path { class: "commits-line chart-line", d: "{curve(&points)}", path_length: "1" }
                        for (& (x , y) , (month , n)) in points.iter().zip(MONTHS) {
                            {
                                let text = format!("{n} in {month}");
                                let tap = text.clone();
                                let (left, top) = (x / CHART_W * 100.0, y / CHART_H * 100.0);
                                rsx! {
                                    g { key: "m{month}",
                                        rect {
                                            class: "commits-hit",
                                            x: "{x - slot / 2.0}",
                                            y: "{CHART_TOP}",
                                            width: "{slot}",
                                            height: "{span_y}",
                                            onmouseenter: move |_| tip.set(Some((text.clone(), left, top))),
                                            onclick: move |_| tip.set(Some((tap.clone(), left, top))),
                                        }
                                        circle { class: "commits-dot kit-dot", cx: "{x}", cy: "{y}", r: "3" }
                                    }
                                }
                            }
                        }
                    }
                    if let Some((text, left, top)) = tip() {
                        ChartTip { text, left, top, class: "tag tag-p1" }
                    }
                }
            }
        }
    }
}

#[component]
fn DiagramShowcase() -> Element {
    rsx! {
        Panel {
            title: "Diagram",
            actions: rsx! {
                Source { path: "src/diagram.rs" }
                Source { path: "assets/components.css#L1391-L1479" }
            },
            p { class: "kit-use",
                "Boxes and arrows drawn as SVG, so the text is real text and the colors follow the theme."
            }
            div { class: "diagram-scroll kit-diagram",
                svg {
                    class: "diagram",
                    view_box: "0 0 520 200",
                    role: "img",
                    "aria-label": "zwipe, portfolio and cairn each pin a commit of orangui",
                    DiagramDefs {}
                    DiagramNode { x: 10.0, y: 74.0, title: "orangui", sub: "the kit", tone: DiagramTone::Primary }
                    DiagramNode { x: 360.0, y: 4.0, title: "zwipe", sub: "app and site", tone: DiagramTone::Secondary }
                    DiagramNode { x: 360.0, y: 74.0, title: "portfolio", sub: "this site", tone: DiagramTone::Tertiary }
                    DiagramNode { x: 360.0, y: 144.0, title: "cairn", sub: "counter app", tone: DiagramTone::Success }
                    DiagramArrow { x1: 358.0, y1: 30.0, x2: 162.0, y2: 92.0 }
                    DiagramArrow { x1: 358.0, y1: 100.0, x2: 162.0, y2: 100.0, label: "pins a commit" }
                    DiagramArrow { x1: 358.0, y1: 170.0, x2: 162.0, y2: 108.0 }
                }
            }
        }
    }
}

/// How each heat level reads in the chip, quietest first.
const BUSY: [&str; 5] = ["nothing", "a little", "some", "a lot", "flat out"];

/// The weekday of a heat grid row, Sunday first.
const WEEKDAYS: [&str; 7] = ["Sun", "Mon", "Tue", "Wed", "Thu", "Fri", "Sat"];

#[component]
fn HeatShowcase() -> Element {
    let cells = use_hook(year_of_days);
    let mut tip = use_signal(|| None::<(String, f64, f64)>);
    let width = heat_span(WEEKS);
    let height = heat_span(HEAT_ROWS);
    let place = {
        let cells = cells.clone();
        move |hit: HeatHit| {
            let Some(cell) = cells.get(hit.index) else {
                return;
            };
            let day = WEEKDAYS.get(cell.row).copied().unwrap_or("");
            let busy = BUSY.get(usize::from(cell.level)).copied().unwrap_or("");
            let text = format!("Week {}, {day}: {busy}", cell.column + 1);
            let left = (hit.x + HEAT_CELL / 2.0) / width * 100.0;
            let top = hit.y / height * 100.0;
            tip.set(Some((text, left, top)));
        }
    };
    let tap = place.clone();
    rsx! {
        Panel {
            title: "HeatGrid",
            actions: rsx! {
                Source { path: "src/heat_grid.rs" }
                Source { path: "assets/components.css#L1593-L1688" }
            },
            p { class: "kit-use",
                "A year of days, one cell each, shaded by how busy the day was. The busiest few get a lit edge."
            }
            div { class: "chart-scroll scroll-fade-x kit-heat",
                div { class: "kit-heat-plot", onmouseleave: move |_| tip.set(None),
                    svg {
                        class: "heat-grid",
                        view_box: "0 0 {width} {height}",
                        role: "img",
                        "aria-label": "A made-up year of days, shaded by how busy each was",
                        HeatGrid {
                            cells: cells.clone(),
                            left: 0.0,
                            top: 0.0,
                            on_enter: place,
                            on_tap: tap,
                        }
                    }
                    if let Some((text, left, top)) = tip() {
                        ChartTip { text, left, top, class: "tag tag-p5" }
                    }
                }
            }
        }
    }
}
