//! Catch-all 404 page.
//!
//! The deploy copies `index.html` to `404.html`, so GitHub Pages boots the
//! app shell for any unknown path and the router lands here (same trick as
//! zite). Not prerendered: catch-all segments are dynamic, so
//! `static_routes()` excludes it automatically. Deliberately a dead end; the
//! nav is right there for anyone who wants to explore.

use dioxus::prelude::*;
use zwipe_components::Panel;

use crate::components::page_meta::PageMeta;

#[component]
pub fn NotFound(segments: Vec<String>) -> Element {
    rsx! {
        PageMeta {
            title: "Page not found",
            description: "Nothing lives at this address.",
            path: "/404",
        }
        // Keep dead paths out of search results.
        document::Meta { name: "robots", content: "noindex" }
        // The shared Panel already is this shape (title, rule, body); the
        // wrapper only sets the narrow width and the dead-end voice, so the
        // card chrome and the rule come from the component like everywhere
        // else. Deliberately a dead end: the nav is right there.
        div { class: "not-found-page content-enter",
            Panel {
                eyebrow: "404",
                title: "Page not found",
                title_h1: true,
                p { class: "card-summary",
                    "Nothing lives at this address. It may have moved with a site update, or the link may be incomplete."
                }
            }
        }
    }
}
