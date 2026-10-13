//! Orangui's app pieces in a phone drawn in CSS: a page header, a body, an
//! action bar whose buttons open a bottom sheet and the theme sheet and fire
//! toasts.
//!
//! The pieces place themselves against the viewport with `position: fixed`.
//! The phone's screen carries a `transform`, which makes it their containing
//! block, and clips them, so a sheet rises from the phone's bottom edge and a
//! toast lands at its top. The screen also carries its own theme class, from
//! a theme signal of its own: a pick in the theme sheet changes the phone and
//! nothing around it.

use super::orangui_showcase::Source;
use dioxus::prelude::*;
use dioxus_primitives::toast::{ToastOptions, use_toast};
use orangui::{
    ActionBar, BottomSheet, Button, ButtonVariant, Chip, PageHeader, Panel, TOAST_NORMAL,
    ThemeConfig, ThemeSheet, ToastStack, display_theme_name, use_theme_follow,
};

/// The element that carries the phone's theme class, for the theme wipe.
const SCREEN: &str = ".phone-screen";

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

/// The phone in a panel, with a line on what each button does.
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
                div { class: "kit-phone-copy",
                    p { class: "kit-use",
                        "The pieces the phone apps share: PageHeader across the top, ActionBar at the bottom, BottomSheet, ThemeSheet and ToastStack. They run here as they do on a phone."
                    }
                    p { class: "kit-note",
                        "Sheet opens a bottom sheet. Theme opens the theme sheet, and a pick there changes only the phone. Toast fires one; fire three and they stack, then tap the stack to spread it out."
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
    // The phone's own theme, apart from the site's.
    let theme = use_signal(ThemeConfig::default);
    let follow = use_theme_follow(theme, SCREEN);
    use_context_provider(|| follow);
    let class = follow.shown.read().css_class();
    rsx! {
        div { class: "phone",
            div { class: "phone-screen {class}",
                ToastStack {
                    PhoneApp { theme }
                }
            }
        }
    }
}

#[component]
fn PhoneApp(theme: Signal<ThemeConfig>) -> Element {
    let toasts = use_toast();
    let mut sheet_open = use_signal(|| false);
    let mut themes_open = use_signal(|| false);
    let mut fired = use_signal(|| 0usize);
    let mut tags = use_signal(|| [true, false, true, false]);
    let current = theme.read().clone();
    let name = display_theme_name(&current.name);
    let mode = if current.is_dark { "Dark" } else { "Light" };

    rsx! {
        div { class: "phone-app",
            div { class: "phone-status" }
            PageHeader { title: "Orangui".to_string(), leading: rsx! {} }
            div { class: "screen-content",
                div { class: "settings-sections",
                    div { class: "settings-list",
                        div { class: "settings-list-header",
                            span { class: "settings-list-title", "This phone" }
                        }
                        div { class: "settings-row",
                            span { class: "settings-row-label", "Theme" }
                            span { class: "settings-row-value", "{name}" }
                        }
                        div { class: "settings-row",
                            span { class: "settings-row-label", "Mode" }
                            span { class: "settings-row-value", "{mode}" }
                        }
                        div { class: "settings-row",
                            span { class: "settings-row-label", "Toasts" }
                            span { class: "settings-row-value", "{fired}" }
                        }
                    }
                    p { class: "pref-note", "The page around the phone keeps its own theme." }
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
                    onclick: move |_| themes_open.set(true),
                    "Theme"
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
            theme,
            on_save: move |_| {
                toasts.success("Theme saved".to_string(), ToastOptions::new().duration(TOAST_NORMAL));
            },
            on_unchanged: |()| {},
        }
    }
}
