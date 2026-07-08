use dioxus::prelude::*;

use crate::theme::{ThemeConfig, THEMES};

const COLORBLIND: &[&str] = &["deuteranopia", "protanopia", "tritanopia"];

#[component]
pub fn ThemeSwitcher() -> Element {
    let mut theme = use_context::<Signal<ThemeConfig>>();
    let mut open = use_signal(|| false);
    let current_name = theme.read().name.clone();
    let is_dark = theme.read().is_dark;
    let select_class = if open() {
        "theme-select theme-select-open"
    } else {
        "theme-select"
    };

    rsx! {
        if open() {
            div {
                class: "theme-backdrop",
                onclick: move |_| open.set(false),
            }
        }
        div { class: "theme-switcher",
            div { class: "{select_class}",
                button {
                    class: "theme-select-trigger",
                    aria_expanded: "{open()}",
                    onclick: move |evt| {
                        evt.stop_propagation();
                        let next = !open();
                        open.set(next);
                    },
                    "{display_name(&current_name)} \u{25be}"
                }
                div { class: "theme-select-content",
                    div { class: "theme-select-label", "Themes" }
                    {THEMES.iter().filter(|entry| !COLORBLIND.contains(&entry.0)).map(|entry| {
                        let id = entry.0;
                        let label = entry.1;
                        let is_active = current_name == id;
                        rsx! {
                            button {
                                class: if is_active { "theme-option active" } else { "theme-option" },
                                onclick: move |_| {
                                    theme.with_mut(|t| t.name = id.to_string());
                                    open.set(false);
                                },
                                "{label}"
                            }
                        }
                    })}
                    div { class: "theme-select-label", "Color blind" }
                    {THEMES.iter().filter(|entry| COLORBLIND.contains(&entry.0)).map(|entry| {
                        let id = entry.0;
                        let label = entry.1;
                        let is_active = current_name == id;
                        rsx! {
                            button {
                                class: if is_active { "theme-option active" } else { "theme-option" },
                                onclick: move |_| {
                                    theme.with_mut(|t| t.name = id.to_string());
                                    open.set(false);
                                },
                                "{label}"
                            }
                        }
                    })}
                }
            }
            button {
                class: "mode-toggle",
                onclick: move |_| {
                    theme.with_mut(|t| t.is_dark = !t.is_dark);
                },
                if is_dark { "light" } else { "dark" }
            }
        }
    }
}

fn display_name(id: &str) -> &'static str {
    THEMES
        .iter()
        .find(|(k, _)| *k == id)
        .map(|(_, v)| *v)
        .unwrap_or("Theme")
}
