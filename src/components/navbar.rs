use dioxus::{document::eval, prelude::*};
use zwipe_components::{BRAND_RESET_JS, NavBar, NavDropdown, ThemeConfig, ThemePicker};

use crate::Route;

const LOGO_S: &str = include_str!("../../assets/s.txt");

#[component]
pub fn Navbar() -> Element {
    let theme = use_context::<Signal<ThemeConfig>>();
    let mut open = use_signal(|| false);
    let mut projects_open = use_signal(|| false);

    rsx! {
        NavBar {
            open,
            brand: rsx! {
                Link {
                    to: Route::Home {},
                    class: "nav-brand",
                    onclick: move |_| {
                        open.set(false);
                        projects_open.set(false);
                        spawn(async {
                            let _ = eval(BRAND_RESET_JS).await;
                        });
                    },
                    pre { class: "nav-logo", "{LOGO_S}" }
                }
            },
            links: rsx! {
                li {
                    NavDropdown {
                        open: projects_open,
                        label: "Projects",
                        Link {
                            to: Route::ProjectDetail { slug: "zwipe".to_string() },
                            class: "nav-dropdown-item",
                            onclick: move |_| {
                                projects_open.set(false);
                                open.set(false);
                            },
                            "Zwipe"
                        }
                        Link {
                            to: Route::ProjectDetail { slug: "halo-action-importer".to_string() },
                            class: "nav-dropdown-item",
                            onclick: move |_| {
                                projects_open.set(false);
                                open.set(false);
                            },
                            "Halo Action Importer"
                        }
                        Link {
                            to: Route::ProjectDetail { slug: "halo-custom-field-builder".to_string() },
                            class: "nav-dropdown-item",
                            onclick: move |_| {
                                projects_open.set(false);
                                open.set(false);
                            },
                            "Halo Custom Field Builder"
                        }
                    }
                }
                li {
                    Link {
                        to: Route::SideQuests {},
                        class: "nav-link",
                        onclick: move |_| open.set(false),
                        "Side Quests"
                    }
                }
                li {
                    Link {
                        to: Route::Contribute {},
                        class: "nav-link",
                        onclick: move |_| open.set(false),
                        "Contribute"
                    }
                }
            },
            trailing: rsx! {
                ThemePicker { theme }
            },
        }
    }
}
