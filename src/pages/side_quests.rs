use dioxus::prelude::*;
use zwipe_components::Panel;

use crate::{Route, components::page_meta::PageMeta, data};

#[component]
pub fn SideQuests() -> Element {
    let quests = data::side_quests();
    rsx! {
        PageMeta {
            title: "Side Quests",
            description: "Proofs of concept and learning projects in Rust: LSM-tree KV store, Redis-compatible server, V4L2 camera capture, ML inference, and more.",
            path: "/side-quests",
        }
        div { class: "side-quests content-enter",
            section { class: "page-header",
                h1 { "Side Quests" }
                p { class: "page-subtitle",
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
