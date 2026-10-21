use dioxus::prelude::*;
use orangui::Panel;

use crate::{
    Route,
    components::{changelog::Changelog, page_meta::PageMeta},
};

/// Cairn's release history. Linked from the cairn project page.
pub const CAIRN_CHANGELOG_PATH: &str = "/projects/cairn/changelog";

/// Cairn's store listing. Linked here and from the cairn page.
pub const CAIRN_APP_STORE_URL: &str = "https://apps.apple.com/us/app/cairn-counter/id6824349629";

#[component]
pub fn CairnChangelog() -> Element {
    rsx! {
        PageMeta {
            title: "Cairn changelog",
            description: "Every Cairn release, newest first: what changed in each version of the app.",
            path: CAIRN_CHANGELOG_PATH,
        }
        div { class: "index-page content-enter",
            Panel {
                eyebrow: "Cairn",
                title: "Changelog",
                // Page hero, so the title is this page's h1 (see detail.rs).
                title_h1: true,
                actions: rsx! {
                    a { href: CAIRN_APP_STORE_URL, class: "panel-action",
                        "App Store" span { class: "ext", "\u{2197}" }
                    }
                    Link {
                        to: Route::ProjectDetail { slug: "cairn".to_string() },
                        class: "panel-action",
                        "About Cairn"
                    }
                },
                p { class: "card-summary", "Every release, newest first." }
            }
            Changelog {}
        }
    }
}
