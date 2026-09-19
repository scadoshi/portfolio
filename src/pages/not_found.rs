//! Catch-all 404 page. Not prerendered: the segments are dynamic, so
//! `static_routes()` skips it.

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
        // The wrapper only sets the narrow width; card chrome comes from Panel.
        // Deliberately a dead end, the nav is right there.
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
