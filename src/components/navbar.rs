use dioxus::{document::eval, prelude::*};
use zwipe_components::{
    BRAND_RESET_JS, Decode, NavBar, NavDropdown, Replay, ThemeConfig, ThemePicker, use_theme_wipe,
};

use crate::{Route, data::Section};

const LOGO_S: &str = include_str!("../../assets/s.txt");

#[component]
pub fn Navbar() -> Element {
    let (theme, shown) = use_theme_wipe(use_context::<Signal<ThemeConfig>>(), ".theme-wrapper");
    let mut open = use_signal(|| false);
    // One per section, in Section::ALL order.
    let mut section_open = [
        use_signal(|| false),
        use_signal(|| false),
        use_signal(|| false),
        use_signal(|| false),
    ];
    let mut replay = use_context::<Replay>().0;
    let mut hovering = use_signal(|| false);

    rsx! {
        NavBar {
            open,
            brand: rsx! {
                Link {
                    to: Route::Home {},
                    class: "nav-brand",
                    onclick: move |_| {
                        open.set(false);
                        for menu in &mut section_open {
                            menu.set(false);
                        }
                        // On the home page this runs the entrance again.
                        replay += 1;
                        spawn(async {
                            let _ = eval(BRAND_RESET_JS).await;
                        });
                    },
                    pre {
                        class: "nav-logo",
                        onmouseenter: move |_| hovering.set(true),
                        onmouseleave: move |_| hovering.set(false),
                        Decode { text: LOGO_S, hover: hovering }
                    }
                }
            },
            links: rsx! {
                // Built from data so the dropdowns cannot drift from the
                // cards, the sitemap and the routes SSG renders. The
                // diprotodon/nighthawk rename is the kind of change that
                // used to leave a list like this pointing at a dead slug.
                for (section, mut menu) in Section::ALL.into_iter().zip(section_open) {
                    li { key: "{section.slug()}",
                        NavDropdown {
                            open: menu,
                            label: section.name(),
                            for project in section.projects() {
                                Link {
                                    key: "{project.slug}",
                                    to: Route::ProjectDetail { slug: project.slug.to_string() },
                                    class: "nav-dropdown-item",
                                    onclick: move |_| {
                                        menu.set(false);
                                        open.set(false);
                                    },
                                    "{project.name}"
                                }
                            }
                            Link {
                                to: Route::SectionPage { section },
                                class: "nav-dropdown-item nav-dropdown-all",
                                onclick: move |_| {
                                    menu.set(false);
                                    open.set(false);
                                },
                                "All {section.name()}"
                            }
                        }
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
                ThemePicker { theme, shown }
            },
        }
    }
}
