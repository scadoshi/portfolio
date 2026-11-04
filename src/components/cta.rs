//! The site's outbound calls to action, GitHub, X, LinkedIn and Email, as icon
//! pills: the kit's `nav-cta-icon` with the mark inside and the name as the
//! link's label and tooltip. The nav bar and the footer draw the same list.

use dioxus::prelude::*;

/// An outbound link the nav and the footer show as an icon pill.
#[derive(Clone, Copy, PartialEq, Debug)]
pub(crate) struct Cta {
    /// The link's accessible name and tooltip.
    pub label: &'static str,
    pub href: &'static str,
    /// A filled 24-unit icon path, drawn in the pill's color.
    pub path: &'static str,
}

/// The calls to action, left to right: GitHub, X, LinkedIn, Email. The nav
/// shows them beside the theme picker and the footer under the copyright.
pub(crate) const CTAS: [Cta; 4] = [
    Cta {
        label: "GitHub",
        href: "https://github.com/scadoshi",
        path: GITHUB,
    },
    Cta {
        label: "X",
        href: "https://x.com/scadoshi",
        path: X,
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

/// The X mark (Simple Icons).
const X: &str = "M18.901 1.153h3.68l-8.04 9.19L24 22.846h-7.406l-5.8-7.584-6.638 7.584H.474l8.6-9.83L0 1.154h7.594l5.243 6.932ZM17.61 20.644h2.039L6.486 3.24H4.298Z";

/// The LinkedIn mark (Simple Icons).
const LINKEDIN: &str = "M20.447 20.452h-3.554v-5.569c0-1.328-.027-3.037-1.852-3.037-1.853 0-2.136 1.445-2.136 2.939v5.667H9.351V9h3.414v1.561h.046c.477-.9 1.637-1.85 3.37-1.85 3.601 0 4.267 2.37 4.267 5.455v6.286zM5.337 7.433c-1.144 0-2.063-.926-2.063-2.065 0-1.138.92-2.063 2.063-2.063 1.14 0 2.064.925 2.064 2.063 0 1.139-.925 2.065-2.064 2.065zm1.782 13.019H3.555V9h3.564v11.452zM22.225 0H1.771C.792 0 0 .774 0 1.729v20.542C0 23.227.792 24 1.771 24h20.451C23.2 24 24 23.227 24 22.271V1.729C24 .774 23.2 0 22.222 0h.003z";

/// A closed envelope: the flap, then the body under the fold.
const ENVELOPE: &str = "M2 5h20v2.4l-10 6.3L2 7.4zM2 9.8l10 6.3 10-6.3V19H2z";

/// One CTA as the kit's icon pill: the SVG inside, the label as its name.
#[component]
pub(crate) fn CtaIcon(cta: Cta, onclick: Option<EventHandler<MouseEvent>>) -> Element {
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
