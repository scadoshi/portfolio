//! Orangui's app pieces in a phone drawn in CSS: a page header, a settings
//! list whose Theme and Mode rows open the theme sheet and flip dark and
//! light, and an action bar whose buttons open a bottom sheet and fire toasts.
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
    ActionBar, BottomSheet, Button, ButtonVariant, Chip, DarkModeButton, PageHeader, Panel,
    TOAST_NORMAL, ThemeConfig, ThemeSheet, ToastStack, display_theme_name, use_theme_follow,
};

/// A selector that matches nothing. The theme sheet reads a follower of the
/// theme it writes; the site's wipe already sweeps the phone, so this one
/// commits each pick with no sweep of its own.
const NO_WIPE: &str = ".phone-no-wipe";

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
                        li { "Sheet slides a bottom sheet up from the bottom edge." }
                        li { "Theme and Mode sit in the settings list, as in the apps; changes apply to the whole site." }
                        li { "Toast fires a notification; send three and they stack, tap to spread them." }
                        li { "The header and the button bar are the same ones the apps use." }
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
    let ThemeWipe { picked, .. } = use_context();
    let follow = use_theme_follow(picked, NO_WIPE);
    use_context_provider(|| follow);
    let class = site.read().css_class();
    rsx! {
        div { class: "phone",
            div { class: "phone-screen {class}",
                ToastStack {
                    PhoneApp { site, picked }
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
                            span { class: "settings-list-title", "This phone" }
                        }
                        div { class: "settings-row",
                            span { class: "settings-row-label", "Theme" }
                            div { class: "settings-row-value",
                                span { "{name}" }
                                Button {
                                    variant: ButtonVariant::Action,
                                    onclick: move |_| themes_open.set(true),
                                    "Change"
                                }
                            }
                        }
                        div { class: "settings-row",
                            span { class: "settings-row-label", "Mode" }
                            div { class: "settings-row-value",
                                DarkModeButton { is_dark: current.is_dark, onclick: toggle_dark }
                            }
                        }
                        div { class: "settings-row",
                            span { class: "settings-row-label", "Toasts" }
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
            on_save: move |_| {
                toasts.success("Theme saved".to_string(), ToastOptions::new().duration(TOAST_NORMAL));
            },
            on_unchanged: |()| {},
        }
    }
}
