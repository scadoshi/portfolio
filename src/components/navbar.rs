use dioxus::document::eval;
use dioxus::prelude::*;
use zwipe_components::{ThemeConfig, ThemePicker};

use crate::Route;

const LOGO_S: &str = include_str!("../../assets/s.txt");

#[component]
pub fn Navbar() -> Element {
    let theme = use_context::<Signal<ThemeConfig>>();
    let mut open = use_signal(|| false);
    let mut projects_open = use_signal(|| false);
    let panel_class = if open() {
        "nav-panel nav-panel-open"
    } else {
        "nav-panel"
    };
    let toggle_class = if open() {
        "nav-toggle nav-toggle-open"
    } else {
        "nav-toggle"
    };
    let projects_class = if projects_open() {
        "dropdown dropdown-open"
    } else {
        "dropdown"
    };

    rsx! {
        nav { class: "navbar",
            div { class: "nav-container",
                Link {
                    to: Route::Home {},
                    class: "nav-brand",
                    onclick: move |_| {
                        open.set(false);
                        projects_open.set(false);
                        spawn(async {
                            let _ = eval(r#"
                                window.scrollTo({ top: 0, behavior: 'smooth' });
                                const el = document.querySelector('.logo');
                                if (el) {
                                    el.style.animation = 'none';
                                    void el.offsetHeight;
                                    el.style.animation = '';
                                }
                            "#).await;
                        });
                    },
                    pre { class: "nav-logo", "{LOGO_S}" }
                }
                button {
                    class: "{toggle_class}",
                    aria_label: "Toggle navigation menu",
                    aria_expanded: "{open()}",
                    onclick: move |_| {
                        let next = !open();
                        open.set(next);
                    },
                    span { class: "nav-toggle-bar" }
                    span { class: "nav-toggle-bar" }
                    span { class: "nav-toggle-bar" }
                }
                div { class: "{panel_class}",
                    div { class: "nav-panel-inner",
                    div { class: "nav-links",
                        if projects_open() {
                            div {
                                class: "nav-backdrop",
                                onclick: move |_| projects_open.set(false),
                            }
                        }
                        div { class: "{projects_class}",
                            button {
                                class: "nav-link dropdown-trigger",
                                aria_expanded: "{projects_open()}",
                                onclick: move |evt| {
                                    evt.stop_propagation();
                                    let next = !projects_open();
                                    projects_open.set(next);
                                },
                                "Projects \u{25be}"
                            }
                            div { class: "dropdown-content",
                                Link {
                                    to: Route::ProjectDetail { slug: "zwipe".to_string() },
                                    class: "dropdown-item",
                                    onclick: move |_| {
                                        projects_open.set(false);
                                        open.set(false);
                                    },
                                    "Zwipe"
                                }
                                Link {
                                    to: Route::ProjectDetail { slug: "halo-action-importer".to_string() },
                                    class: "dropdown-item",
                                    onclick: move |_| {
                                        projects_open.set(false);
                                        open.set(false);
                                    },
                                    "Halo Action Importer"
                                }
                                Link {
                                    to: Route::ProjectDetail { slug: "halo-custom-field-builder".to_string() },
                                    class: "dropdown-item",
                                    onclick: move |_| {
                                        projects_open.set(false);
                                        open.set(false);
                                    },
                                    "Halo Custom Field Builder"
                                }
                            }
                        }
                        Link {
                            to: Route::SideQuests {},
                            class: "nav-link",
                            onclick: move |_| open.set(false),
                            "Side Quests"
                        }
                        Link {
                            to: Route::Contribute {},
                            class: "nav-link",
                            onclick: move |_| open.set(false),
                            "Contribute"
                        }
                    }
                    ThemePicker { theme }
                    } // nav-panel-inner
                }
            }
        }
    }
}
