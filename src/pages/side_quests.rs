use dioxus::prelude::*;
use zwipe_components::Panel;

use crate::{Route, components::page_meta::PageMeta, data};

#[component]
pub fn SideQuests() -> Element {
    let quests = data::side_quests();
    rsx! {
        PageMeta {
            title: "Side Quests",
            description: "Side quests: an LSM-tree KV store, a Redis-compatible server, camera capture, on-device ML inference, and Advent of Code tooling in Rust and C#.",
            path: "/side-quests",
        }
        div { class: "side-quests content-enter",
            Panel {
                eyebrow: "Explore",
                title: "Side Quests",
                // Page hero, so the title is this page's h1 (see detail.rs).
                title_h1: true,
                p { class: "card-summary",
                    "Proofs of concept and learning projects. Each one explores a domain I wanted to understand by building something real."
                }
            }
            div { class: "projects-grid",
                for quest in quests {
                    Panel {
                        eyebrow: quest.category.to_string(),
                        title: quest.name.to_string(),
                        status: quest.status.banner_status(),
                        status_label: quest.status.label().to_string(),
                        actions: rsx! {
                            Link {
                                to: Route::SideQuestDetail { slug: quest.slug.to_string() },
                                class: "panel-action",
                                "View Side Quest"
                            }
                            a {
                                href: "{quest.repo_url}",
                                class: "panel-action",
                                "GitHub \u{2197}"
                            }
                        },
                        p { class: "card-summary", "{quest.summary}" }
                        ul { class: "card-bullets",
                            for bullet in quest.card_bullets {
                                li { "{bullet}" }
                            }
                        }
                    }
                }
            }
        }
    }
}
