use dioxus::prelude::*;

use crate::components::code_block::CodeBlock;
use crate::data;

#[component]
pub fn SideQuests() -> Element {
    let quests = data::side_quests();
    rsx! {
        div { class: "side-quests",
            section { class: "page-header",
                h1 { "Side Quests" }
                p { class: "page-subtitle",
                    "Learning projects \u{2014} going deep on things I don't know."
                }
            }
            for quest in quests {
                section { class: "quest-section",
                    div { class: "quest-header",
                        h2 { "{quest.name}" }
                        span { class: "quest-category", "{quest.category}" }
                        a {
                            href: "{quest.repo_url}",
                            target: "_blank",
                            rel: "noopener noreferrer",
                            class: "repo-link",
                            "GitHub \u{2192}"
                        }
                    }
                    p { class: "quest-description", "{quest.description}" }
                    ul { class: "quest-highlights",
                        for highlight in quest.highlights {
                            li { "{highlight}" }
                        }
                    }
                    CodeBlock {
                        title: quest.snippet_title.to_string(),
                        code: quest.snippet_code.to_string(),
                        description: String::new(),
                    }
                }
            }
        }
    }
}
