use dioxus::prelude::*;

use crate::theme::{ThemeConfig, THEMES};

#[component]
pub fn ThemeSwitcher() -> Element {
    let mut theme = use_context::<Signal<ThemeConfig>>();
    let current_name = theme.read().name.clone();
    let is_dark = theme.read().is_dark;
    let has_light = theme.read().has_light_mode();

    rsx! {
        div { class: "theme-switcher",
            div { class: "theme-select",
                span { class: "theme-select-trigger",
                    "{display_name(&current_name)} \u{25be}"
                }
                div { class: "theme-select-content",
                    {THEMES.iter().map(|entry| {
                        let id = entry.0;
                        let label = entry.1;
                        let is_active = current_name == id;
                        rsx! {
                            button {
                                class: if is_active { "theme-option active" } else { "theme-option" },
                                onclick: move |_| {
                                    theme.with_mut(|t| {
                                        t.name = id.to_string();
                                        if id == "vantablack" {
                                            t.is_dark = true;
                                        }
                                    });
                                },
                                "{label}"
                            }
                        }
                    })}
                }
            }
            if has_light {
                button {
                    class: "mode-toggle",
                    onclick: move |_| {
                        theme.with_mut(|t| t.is_dark = !t.is_dark);
                    },
                    if is_dark { "[light]" } else { "[dark]" }
                }
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
