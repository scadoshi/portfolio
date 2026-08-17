//! Catch-all 404 page.
//!
//! The deploy copies `index.html` to `404.html`, so GitHub Pages boots the
//! app shell for any unknown path and the router lands here (same trick as
//! zite). Not prerendered: catch-all segments are dynamic, so
//! `static_routes()` excludes it automatically. Deliberately a dead end; the
//! nav is right there for anyone who wants to explore.

use dioxus::prelude::*;

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
        // Formatted like zite's dead-link state (inform-and-stop alert):
        // deliberately a dead end, the nav is right there for anyone who
        // wants to explore.
        div { class: "form-page content-enter",
            span { class: "sd-alert-title", "Page not found" }
            hr {
                style: "border: none; border-top: 1px solid var(--border-secondary); margin: 0.75rem -2rem;",
            }
            p { class: "subtitle", style: "text-align: center; margin-bottom: 0;",
                "Nothing lives at this address. It may have moved with a site update, or the link may be incomplete."
            }
        }
    }
}
