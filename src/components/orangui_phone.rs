//! Orangui's app pieces in a phone drawn in CSS: a page header, a settings
//! list whose Theme row opens the theme sheet and flips dark and light, and
//! an action bar whose buttons open a bottom sheet and fire toasts.
//!
//! The pieces place themselves against the viewport with `position: fixed`.
//! The phone's screen carries a `transform`, which makes it their containing
//! block, and clips them, so a sheet rises from the phone's bottom edge and a
//! toast lands at its top. The phone and the site share one theme: a pick in
//! the phone goes through the site's wipe, as a pick in the nav does, and the
//! screen's theme class follows the site's.

use super::{navbar::ThemeWipe, orangui_showcase::Source};
use dioxus::prelude::*;
use dioxus_primitives::toast::{ToastOptions, use_toast};
use orangui::{
    ActionBar, AlertDialog, BottomSheet, Button, ButtonVariant, Chip, DarkModeButton, DialogAction,
    DialogHostView, InfoButton, PageHeader, Panel, TOAST_NORMAL, ThemeConfig, ThemeSheet,
    ToastStack, display_theme_name, use_dialog_host,
};

/// The hint the phone's "?" buttons open: a title and a line, as the apps'
/// settings hints are. Shared by every row through context.
#[derive(Clone, Copy)]
struct PhoneHint {
    open: Signal<bool>,
    which: Signal<(&'static str, &'static str)>,
}

impl PhoneHint {
    fn show(mut self, title: &'static str, body: &'static str) {
        self.which.set((title, body));
        self.open.set(true);
    }
}

const THEME_HINT: (&str, &str) = (
    "Theme",
    "Picks the colors. Change opens every theme; the sun or moon flips light and dark.",
);

/// Sets the phone's `--phone-scale` to its viewport's width over the 414
/// points of an iPhone 11 Pro Max, the size the app lays out at, and keeps it
/// in step as the page resizes.
const SCALE_JS: &str = r#"
const view = document.querySelector(".phone-viewport");
if (view) {
    const fit = () => view.style.setProperty("--phone-scale", view.clientWidth / 414);
    fit();
    new ResizeObserver(fit).observe(view);
}
"#;

/// The files behind the phone's pieces, and their stylesheets.
const SOURCES: [&str; 6] = [
    "src/page_header.rs",
    "src/bottom_sheet.rs",
    "src/theme_sheet.rs",
    "src/toast_stack.rs",
    "assets/app.css",
    "assets/toast.css",
];

/// The toasts the phone fires, in turn.
const TOASTS: [(&str, Kind); 3] = [
    ("Saved", Kind::Success),
    ("Link copied", Kind::Info),
    ("Offline, will retry", Kind::Warning),
];

#[derive(Clone, Copy)]
enum Kind {
    Success,
    Info,
    Warning,
}

/// The phone in a panel, with a list of what each button does.
#[component]
pub fn OranguiPhone() -> Element {
    rsx! {
        Panel {
            title: "App kit",
            actions: rsx! {
                for path in SOURCES {
                    Source { key: "{path}", path }
                }
            },
            div { class: "kit-phone-panel",
                div { class: "kit-phone-copy project-section",
                    ul {
                        li { "Sheet slides a panel up from the bottom." }
                        li { "The Theme row works like it does in the apps, and changes the whole site." }
                        li { "Toast sends a notification. Send a few and they stack; tap to spread them." }
                        li { "The header and bottom bar are the same ones the apps use." }
                    }
                }
                Phone {}
            }
        }
    }
}

/// The frame, the screen and the app on it.
#[component]
fn Phone() -> Element {
    // The site's theme. A pick goes to the wipe's `picked`, which sweeps it
    // across the page and commits it here.
    let site = use_context::<Signal<ThemeConfig>>();
    // The theme sheet waits on the site's wipe: Back on a discarded pick
    // keeps it up while the old theme sweeps back, gone on the other side.
    let ThemeWipe { picked, follow, .. } = use_context();
    use_context_provider(|| follow);
    // Its own dialog host, drawn inside the screen, so a hint opens in the
    // phone rather than over the page.
    let host = use_dialog_host();
    use_context_provider(|| host);
    let hint = PhoneHint {
        open: use_signal(|| false),
        which: use_signal(|| THEME_HINT),
    };
    use_context_provider(|| hint);
    let class = site.read().css_class();
    // The app lays out at a real phone's size and the screen scales it down
    // to fit the frame, which CSS cannot work out from the frame's width.
    use_effect(|| {
        document::eval(SCALE_JS);
    });
    rsx! {
        div { class: "phone",
            div { class: "phone-viewport",
                div { class: "phone-screen {class}",
                    ToastStack {
                        PhoneApp { site, picked }
                    }
                    DialogHostView {}
                }
            }
        }
    }
}

#[component]
fn PhoneApp(site: Signal<ThemeConfig>, picked: Signal<ThemeConfig>) -> Element {
    let toasts = use_toast();
    let mut sheet_open = use_signal(|| false);
    let mut themes_open = use_signal(|| false);
    let mut fired = use_signal(|| 0usize);
    let mut tags = use_signal(|| [true, false, true, false]);
    let hint: PhoneHint = use_context();
    let current = site.read().clone();
    let name = display_theme_name(&current.name);
    let mut picked = picked;
    let toggle_dark = move |_| {
        let prev = site.peek().clone();
        picked.set(ThemeConfig {
            name: prev.name,
            is_dark: !prev.is_dark,
        });
    };

    rsx! {
        div { class: "phone-app",
            div { class: "phone-status" }
            PageHeader { title: "Orangui".to_string(), leading: rsx! {} }
            div { class: "screen-content",
                div { class: "settings-sections",
                    div { class: "settings-list",
                        div { class: "settings-list-header",
                            span { class: "settings-list-title", "Appearance" }
                        }
                        div { class: "settings-row",
                            span { class: "row-label-with-hint",
                                span { class: "settings-row-label", "Theme" }
                                InfoButton { onclick: move |_| hint.show(THEME_HINT.0, THEME_HINT.1) }
                            }
                            div { class: "settings-row-value",
                                span { "{name}" }
                                DarkModeButton { is_dark: current.is_dark, onclick: toggle_dark }
                                Button {
                                    variant: ButtonVariant::Action,
                                    onclick: move |_| {
                                        hint.which.clone().set(THEME_HINT);
                                        themes_open.set(true);
                                    },
                                    "Change"
                                }
                            }
                        }
                    }
                    div { class: "settings-list",
                        div { class: "settings-list-header",
                            span { class: "settings-list-title", "When to count" }
                        }
                        CycleRow { label: "Day starts", hint: "When a new day begins, for anyone up past midnight.", options: &["Midnight", "4 AM", "6 AM"] }
                        CycleRow { label: "Week starts", hint: "The first day of each week in the weekly totals.", options: &["Monday", "Sunday"] }
                        CycleRow { label: "Rest days", hint: "Days that never break a streak.", options: &["Weekends", "Sundays", "None"] }
                    }
                    div { class: "settings-list",
                        div { class: "settings-list-header",
                            span { class: "settings-list-title", "Decks" }
                        }
                        CycleRow { label: "Format", hint: "The format a new deck starts in.", options: &["Commander", "Oathbreaker", "Brawl"] }
                        CycleRow { label: "Price target", hint: "What a deck aims to cost. Cards over it are flagged.", options: &["$100", "$250", "Any"] }
                        CycleRow { label: "Confirm remove", hint: "Asks before a card leaves a deck.", options: &["On", "Off"] }
                    }
                    div { class: "settings-list",
                        div { class: "settings-list-header",
                            span { class: "settings-list-title", "About" }
                        }
                        div { class: "settings-row",
                            span { class: "settings-row-label", "Version" }
                            span { class: "settings-row-value", "1.0.0" }
                        }
                        div { class: "settings-row",
                            span { class: "settings-row-label", "Toasts sent" }
                            span { class: "settings-row-value", "{fired}" }
                        }
                    }
                }
            }
            ActionBar {
                Button {
                    variant: ButtonVariant::Action,
                    onclick: move |_| sheet_open.set(true),
                    "Sheet"
                }
                Button {
                    variant: ButtonVariant::Action,
                    onclick: move |_| {
                        let (text, kind) = TOASTS[fired() % TOASTS.len()];
                        let options = ToastOptions::new().duration(TOAST_NORMAL);
                        match kind {
                            Kind::Success => toasts.success(text.to_string(), options),
                            Kind::Info => toasts.info(text.to_string(), options),
                            Kind::Warning => toasts.warning(text.to_string(), options),
                        }
                        fired += 1;
                    },
                    "Toast"
                }
            }
        }
        BottomSheet { open: sheet_open, title: "Tags".to_string(),
            p { class: "pref-note", "A sheet holds a short task without leaving the screen." }
            div { class: "kit-row kit-sheet-chips",
                for (i, label) in ["rust", "dioxus", "wasm", "css"].into_iter().enumerate() {
                    Chip {
                        key: "{label}",
                        selected: tags.read().get(i).copied().unwrap_or(false),
                        onclick: move |_| {
                            if let Some(tag) = tags.write().get_mut(i) {
                                *tag = !*tag;
                            }
                        },
                        "{label}"
                    }
                }
            }
        }
        ThemeSheet {
            open: themes_open,
            theme: picked,
            hint: Some(hint.open),
            on_save: move |_| {
                toasts.success("Theme saved".to_string(), ToastOptions::new().duration(TOAST_NORMAL));
            },
            on_unchanged: |()| {},
        }
        AlertDialog {
            open: hint.open,
            title: (hint.which)().0.to_string(),
            actions: vec![DialogAction::Dismiss("Got it".to_string())],
            p { class: "phone-hint-body", "{(hint.which)().1}" }
        }
    }
}

/// A made-up setting like the apps' own: its value is the button, a tap moves
/// to the next one and says so in a toast, and its "?" explains it.
#[component]
fn CycleRow(label: &'static str, hint: &'static str, options: &'static [&'static str]) -> Element {
    let toasts = use_toast();
    let phone_hint: PhoneHint = use_context();
    let mut at = use_signal(|| 0usize);
    let value = options.get(at()).copied().unwrap_or_default();
    rsx! {
        div { class: "settings-row",
            span { class: "row-label-with-hint",
                span { class: "settings-row-label", "{label}" }
                InfoButton { onclick: move |_| phone_hint.show(label, hint) }
            }
            div { class: "settings-row-value",
                Button {
                    variant: ButtonVariant::Action,
                    onclick: move |_| {
                        let next = (at() + 1) % options.len().max(1);
                        at.set(next);
                        let value = options.get(next).copied().unwrap_or_default();
                        toasts.success(
                            format!("{label}: {value}"),
                            ToastOptions::new().duration(TOAST_NORMAL),
                        );
                    },
                    "{value}"
                }
            }
        }
    }
}
