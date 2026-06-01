use dioxus::prelude::*;

use crate::components::page_meta::PageMeta;
use crate::data;
use crate::Route;

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
                    div { class: "project-card",
                        div { class: "card-category",
                            "{quest.category}"
                            span { class: "status-tag {quest.status.css_class()}", "{quest.status.label()}" }
                        }
                        h3 { class: "card-title", "{quest.name}" }
                        p { class: "card-summary", "{quest.summary}" }
                        ul { class: "card-bullets",
                            for bullet in quest.card_bullets {
                                li { "{bullet}" }
                            }
                        }
                        div { class: "card-actions",
                            Link {
                                to: Route::SideQuestDetail { slug: quest.slug.to_string() },
                                class: "card-link",
                                "View Side Quest \u{2192}"
                            }
                            a {
                                href: "{quest.repo_url}",
                                target: "_blank",
                                rel: "noopener noreferrer",
                                class: "card-link card-link-secondary",
                                "GitHub \u{2192}"
                            }
                        }
                    }
                }
            }
        }
    }
}
