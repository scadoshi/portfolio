use dioxus::prelude::*;

use crate::components::code_block::CodeBlock;
use crate::components::linked_text::LinkedText;
use crate::data;

#[component]
pub fn SideQuestDetail(slug: String) -> Element {
    let Some(project) = data::find_side_quest(&slug) else {
        return rsx! {
            div { class: "not-found",
                h1 { "Side quest not found" }
                p { "No side quest matches \"{slug}\"." }
            }
        };
    };

    rsx! {
        div { class: "project-detail",
            section { class: "project-header",
                span { class: "project-category-tag", "{project.category}" }
                h1 { class: "project-name", "{project.name}" }
                p { class: "project-headline", "{project.headline}" }
                a {
                    href: "{project.repo_url}",
                    target: "_blank",
                    rel: "noopener noreferrer",
                    class: "repo-link",
                    "View on GitHub \u{2192}"
                }
            }

            section { class: "project-section",
                h2 { "Objective" }
                p { LinkedText { text: project.objective.to_string() } }
            }

            section { class: "project-section",
                h2 { "Approach" }
                ul {
                    for point in project.approach {
                        li { "{point}" }
                    }
                }
            }

            section { class: "project-section",
                h2 { "Implementation" }
                for snippet in project.snippets {
                    CodeBlock {
                        key: "{project.slug}-{snippet.title}",
                        title: snippet.title.to_string(),
                        code: snippet.code.to_string(),
                        description: snippet.description.to_string(),
                    }
                }
            }

            section { class: "project-section",
                h2 { "Obstacles" }
                ul {
                    for obstacle in project.obstacles {
                        li { LinkedText { text: obstacle.to_string() } }
                    }
                }
            }

            section { class: "project-section",
                h2 { "Progress & Impact" }
                p { "{project.progress}" }
                p { class: "impact-statement", "{project.impact}" }
            }
        }
    }
}
