use dioxus::prelude::*;

use crate::data;
use crate::Route;

#[component]
pub fn SideQuests() -> Element {
    let quests = data::side_quests();
    rsx! {
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
                        div { class: "card-category", "{quest.category}" }
                        h3 { class: "card-title", "{quest.name}" }
                        p { class: "card-summary", "{quest.summary}" }
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
