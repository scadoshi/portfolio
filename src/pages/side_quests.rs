use dioxus::prelude::*;
use zwipe_components::Panel;

use crate::{Route, components::page_meta::PageMeta, data};

#[component]
pub fn SideQuests() -> Element {
    let quests = data::side_quests();
    rsx! {
        PageMeta {
            title: "Side Quests",
            description: "Rust side quests: an LSM-tree KV store, a Redis-compatible server, camera capture, and on-device ML inference.",
            path: "/side-quests",
        }
        div { class: "side-quests content-enter",
            Panel {
                eyebrow: "Explore",
                title: "Side Quests",
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
                                target: "_blank",
                                rel: "noopener noreferrer",
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
