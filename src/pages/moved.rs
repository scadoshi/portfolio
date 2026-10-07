use dioxus::prelude::*;

use crate::{Route, components::page_meta::PageMeta, data, pages::not_found::NotFound};

/// A page that has moved.
///
/// Prerendered with a refresh and a canonical, which is what a direct visit and a
/// crawler see. A visit from inside the app never loads that HTML, so the route is
/// replaced here as well.
#[component]
pub fn Moved(to: &'static str) -> Element {
    use_effect(move || {
        if let Ok(route) = to.parse::<Route>() {
            navigator().replace(route);
        }
    });
    rsx! {
        PageMeta {
            title: "Moved",
            description: "This page is now at {to}.",
            path: to,
        }
        document::Meta { http_equiv: "refresh", content: "0; url={to}" }
        div { class: "not-found",
            h1 { "Moved" }
            p {
                "This page is now at "
                a { href: "{to}", "{to}" }
                "."
            }
        }
    }
}

/// The redirect for `path` if it moved, else the 404 page.
fn moved_or_missing(path: &str) -> Element {
    match data::moved_to(path) {
        Some(to) => rsx! { Moved { to } },
        // NotFound never reads its segments; the route needs them, not the page.
        None => rsx! { NotFound { segments: Vec::new() } },
    }
}

/// `/side-quests`, now `/projects`.
#[component]
pub fn OldSideQuests() -> Element {
    moved_or_missing("/side-quests")
}

/// `/side-quests/:slug`, now under `/projects`.
#[component]
pub fn OldSideQuest(slug: String) -> Element {
    moved_or_missing(&format!("/side-quests/{slug}"))
}
