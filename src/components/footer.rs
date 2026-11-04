//! The site footer: the copyright line, then the calls to action as chips,
//! the three icon pills and Contribute, and the note on how the site is
//! built, whose links stay links.

use dioxus::prelude::*;

use crate::Route;
use crate::components::cta::{CTAS, CtaIcon};

#[component]
pub fn Footer() -> Element {
    rsx! {
        footer { class: "site-footer",
            p { class: "footer-copy", "\u{00a9} 2026 Scotty Fermo" }
            div { class: "footer-links",
                for cta in CTAS {
                    CtaIcon { key: "{cta.label}", cta }
                }
                Link { to: Route::Contribute {}, class: "nav-cta", "Contribute" }
            }
            p { class: "site-footer-note",
                a {
                    class: "footer-built-repo",
                    href: "https://github.com/scadoshi/portfolio",
                    "This site"
                }
                " is Rust, compiled to WebAssembly with Dioxus. Its numbers come live from "
                a { href: "https://github.com/scadoshi/heron", "Heron" }
                ", counted from each repository after every push, with Heron's last answer built in as a fallback."
            }
        }
    }
}
