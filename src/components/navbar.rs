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
                    // Built from data so the dropdown cannot drift from the
                    // cards, the sitemap and the routes SSG renders. The
                    // diprotodon/nighthawk rename is the kind of change that
                    // used to leave this list pointing at a dead slug.
                    NavDropdown {
                        open: projects_open,
                        label: "Projects",
                        for project in crate::data::featured_projects() {
                            Link {
                                key: "{project.slug}",
                                to: Route::ProjectDetail { slug: project.slug.to_string() },
                                class: "nav-dropdown-item",
                                onclick: move |_| {
                                    projects_open.set(false);
                                    open.set(false);
                                },
                                "{project.name}"
                            }
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
                // Outbound CTAs, zite-style: store-link pills in the warning
                // accent. Hidden in the collapsed panel on mobile, where the
                // persistent copies below cover them.
                li { class: "nav-link-store",
                    a {
                        class: "store-link",
                        href: "https://github.com/scadoshi",
                        onclick: move |_| open.set(false),
                        "GitHub" span { class: "ext", "\u{2197}" }
                    }
                }
                li { class: "nav-link-store",
                    a {
                        class: "store-link",
                        href: "https://www.linkedin.com/in/scotty-fermo-41a35b141/",
                        onclick: move |_| open.set(false),
                        "LinkedIn" span { class: "ext", "\u{2197}" }
                    }
                }
                li { class: "nav-link-store",
                    a {
                        class: "store-link",
                        href: "mailto:scottyfermo@hotmail.com",
                        onclick: move |_| open.set(false),
                        "Email" span { class: "ext", "\u{2197}" }
                    }
                }
            },
            // Mobile: the CTAs stay visible beside the hamburger instead of
            // hiding inside the collapsed panel (same trick as zite).
            persistent: rsx! {
                div { class: "nav-stores-persistent",
                    a {
                        class: "store-link",
                        href: "https://github.com/scadoshi",
                        "GitHub" span { class: "ext", "\u{2197}" }
                    }
                    a {
                        class: "store-link",
                        href: "https://www.linkedin.com/in/scotty-fermo-41a35b141/",
                        "LinkedIn" span { class: "ext", "\u{2197}" }
                    }
                    a {
                        class: "store-link",
                        href: "mailto:scottyfermo@hotmail.com",
                        "Email" span { class: "ext", "\u{2197}" }
                    }
                }
            },
            trailing: rsx! {
                ThemePicker { theme }
            },
        }
    }
}
