use dioxus::prelude::*;
use dioxus::document::eval;

use crate::components::theme_switcher::ThemeSwitcher;
use crate::Route;

const LOGO_S: &str = include_str!("../../assets/s.txt");

#[component]
pub fn Navbar() -> Element {
    rsx! {
        nav { class: "navbar",
            div { class: "nav-container",
                Link {
                    to: Route::Home {},
                    class: "nav-brand",
                    onclick: move |_| {
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
                div { class: "nav-links",
                    Link { to: Route::Home {}, class: "nav-link", "Home" }
                    div { class: "dropdown",
                        span { class: "nav-link dropdown-trigger", "Projects \u{25be}" }
                        div { class: "dropdown-content",
                            Link {
                                to: Route::ProjectDetail { slug: "zwipe".to_string() },
                                class: "dropdown-item",
                                "Zwipe"
                            }
                            Link {
                                to: Route::ProjectDetail { slug: "halo-action-importer".to_string() },
                                class: "dropdown-item",
                                "Halo Action Importer"
                            }
                            Link {
                                to: Route::ProjectDetail { slug: "halo-custom-field-builder".to_string() },
                                class: "dropdown-item",
                                "Halo Custom Field Builder"
                            }
                        }
                    }
                    Link { to: Route::SideQuests {}, class: "nav-link", "Side Quests" }
                    Link { to: Route::Contribute {}, class: "nav-link", "Contribute" }
                }
                ThemeSwitcher {}
            }
        }
    }
}
