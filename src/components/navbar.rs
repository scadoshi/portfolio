use dioxus::document;
use dioxus::prelude::*;
use orangui::{
    BRAND_RESET_JS, Decode, NavBar, NavDropdown, Replay, ThemeConfig, ThemeFollow, ThemePicker,
};

use crate::Route;
use crate::data::Section;

const LOGO_S: &str = include_str!("../../assets/s.txt");

/// An outbound link the nav shows as an icon pill.
#[derive(Clone, Copy, PartialEq, Debug)]
struct Cta {
    /// The link's accessible name and tooltip.
    label: &'static str,
    href: &'static str,
    /// A filled 24-unit icon path, drawn in the pill's color.
    path: &'static str,
}

/// The nav's calls to action, left to right: GitHub, LinkedIn, Email. The
/// footer links the same three by name.
const CTAS: [Cta; 3] = [
    Cta {
        label: "GitHub",
        href: "https://github.com/scadoshi",
        path: GITHUB,
    },
    Cta {
        label: "LinkedIn",
        href: "https://www.linkedin.com/in/scotty-fermo-41a35b141/",
        path: LINKEDIN,
    },
    Cta {
        label: "Email",
        href: "mailto:scottyfermo@hotmail.com",
        path: ENVELOPE,
    },
];

/// The GitHub mark (Simple Icons).
const GITHUB: &str = "M12 .297c-6.63 0-12 5.373-12 12 0 5.303 3.438 9.8 8.205 11.385.6.113.82-.258.82-.577 0-.285-.01-1.04-.015-2.04-3.338.724-4.042-1.61-4.042-1.61C4.422 18.07 3.633 17.7 3.633 17.7c-1.087-.744.084-.729.084-.729 1.205.084 1.838 1.236 1.838 1.236 1.07 1.835 2.809 1.305 3.495.998.108-.776.417-1.305.76-1.605-2.665-.3-5.466-1.332-5.466-5.93 0-1.31.465-2.38 1.235-3.22-.135-.303-.54-1.523.105-3.176 0 0 1.005-.322 3.3 1.23.96-.267 1.98-.399 3-.405 1.02.006 2.04.138 3 .405 2.28-1.552 3.285-1.23 3.285-1.23.645 1.653.24 2.873.12 3.176.765.84 1.23 1.91 1.23 3.22 0 4.61-2.805 5.625-5.475 5.92.42.36.81 1.096.81 2.22 0 1.606-.015 2.896-.015 3.286 0 .315.21.69.825.57C20.565 22.092 24 17.592 24 12.297c0-6.627-5.373-12-12-12";

/// The LinkedIn mark (Simple Icons).
const LINKEDIN: &str = "M20.447 20.452h-3.554v-5.569c0-1.328-.027-3.037-1.852-3.037-1.853 0-2.136 1.445-2.136 2.939v5.667H9.351V9h3.414v1.561h.046c.477-.9 1.637-1.85 3.37-1.85 3.601 0 4.267 2.37 4.267 5.455v6.286zM5.337 7.433c-1.144 0-2.063-.926-2.063-2.065 0-1.138.92-2.063 2.063-2.063 1.14 0 2.064.925 2.064 2.063 0 1.139-.925 2.065-2.064 2.065zm1.782 13.019H3.555V9h3.564v11.452zM22.225 0H1.771C.792 0 0 .774 0 1.729v20.542C0 23.227.792 24 1.771 24h20.451C23.2 24 24 23.227 24 22.271V1.729C24 .774 23.2 0 22.222 0h.003z";

/// A closed envelope: the flap, then the body under the fold.
const ENVELOPE: &str = "M2 5h20v2.4l-10 6.3L2 7.4zM2 9.8l10 6.3 10-6.3V19H2z";

/// One CTA as the kit's icon pill: the SVG inside, the label as its name.
#[component]
fn CtaIcon(cta: Cta, onclick: Option<EventHandler<MouseEvent>>) -> Element {
    rsx! {
        a {
            class: "nav-cta nav-cta-icon",
            href: cta.href,
            "aria-label": cta.label,
            title: cta.label,
            onclick: move |event| {
                if let Some(handler) = &onclick {
                    handler.call(event);
                }
            },
            svg {
                view_box: "0 0 24 24",
                "aria-hidden": "true",
                path { d: cta.path }
            }
        }
    }
}

/// The site's theme wipe, shared by everything that picks a theme: the nav's
/// picker and the phone on the orangui page. `picked` takes a pick and sweeps
/// it across the page; `shown` is what a picker shows as current meanwhile;
/// `follow` is the same wipe for a theme sheet that waits on it.
#[derive(Clone, Copy)]
pub(crate) struct ThemeWipe {
    pub picked: Signal<ThemeConfig>,
    pub shown: Signal<ThemeConfig>,
    pub follow: ThemeFollow,
}

#[component]
pub fn Navbar() -> Element {
    let ThemeWipe {
        picked: theme,
        shown,
        ..
    } = use_context();
    let mut open = use_signal(|| false);
    // One per section, in Section::ALL order.
    let section_open = [
        use_signal(|| false),
        use_signal(|| false),
        use_signal(|| false),
        use_signal(|| false),
    ];
    // Handed to ThemePicker, so its menu takes part in the rule below.
    let theme_open = use_signal(|| false);
    let mut menus = [
        section_open[0],
        section_open[1],
        section_open[2],
        section_open[3],
        theme_open,
    ];
    // One menu at a time. A trigger sits above another menu's click-away
    // backdrop and stops its own click, so opening a second menu never
    // closed the first and they piled over each other. Watch them all
    // instead: whichever opened last closes the rest.
    let mut last_opened = use_signal(|| None::<usize>);
    use_effect(move || {
        let opened: Vec<usize> = menus
            .iter()
            .enumerate()
            .filter(|(_, menu)| menu())
            .map(|(i, _)| i)
            .collect();
        let previous = *last_opened.peek();
        if let Some(&newest) = opened.iter().find(|&&i| Some(i) != previous) {
            for (i, mut menu) in menus.into_iter().enumerate() {
                if i != newest && *menu.peek() {
                    menu.set(false);
                }
            }
            last_opened.set(Some(newest));
        } else if opened.is_empty() && previous.is_some() {
            last_opened.set(None);
        }
    });
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
                        for menu in &mut menus {
                            menu.set(false);
                        }
                        // On the home page this runs the entrance again.
                        replay += 1;
                        spawn(async {
                            let _ = document::eval(BRAND_RESET_JS).await;
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
                // cards, the sitemap and the routes SSG renders. A list kept
                // by hand points at a dead slug after a rename.
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
                                class: "nav-dropdown-item",
                                onclick: move |_| {
                                    menu.set(false);
                                    open.set(false);
                                },
                                "All"
                            }
                        }
                    }
                }
                // Outbound CTAs as icon pills in palette slot 5. Hidden in
                // the collapsed panel on mobile, where the persistent copies
                // below cover them.
                for cta in CTAS {
                    li { key: "{cta.label}", class: "nav-cta-item",
                        CtaIcon { cta, onclick: move |_| open.set(false) }
                    }
                }
            },
            // Mobile: the CTAs stay visible beside the hamburger instead of
            // hiding inside the collapsed panel (same trick as zite).
            persistent: rsx! {
                for cta in CTAS {
                    CtaIcon { key: "{cta.label}", cta }
                }
            },
            trailing: rsx! {
                ThemePicker { theme, shown, open: theme_open }
            },
        }
    }
}
