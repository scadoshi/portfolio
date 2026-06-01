use dioxus::prelude::*;

use crate::components::code_block::CodeBlock;
use crate::components::gallery::ProjectGallery;
use crate::components::linked_text::LinkedText;
use crate::components::page_meta::PageMeta;
use crate::data;

#[component]
pub fn ProjectDetail(slug: String) -> Element {
    let Some(project) = data::find_project(&slug) else {
        return rsx! {
            document::Title { "Project not found | Scotty Fermo" }
            div { class: "not-found",
                h1 { "Project not found" }
                p { "No project matches \"{slug}\"." }
            }
        };
    };

    rsx! {
        PageMeta {
            title: project.name.to_string(),
            description: project.headline.to_string(),
            path: format!("/projects/{}", project.slug),
        }
        div { class: "project-detail content-enter",
            section { class: "project-header",
                span { class: "project-category-tag",
                    "{project.category}"
                    span { class: "status-tag {project.status.css_class()}", "{project.status.label()}" }
                }
                h1 { class: "project-name", "{project.name}" }
                p { class: "project-headline", "{project.headline}" }
                a {
                    href: "{project.repo_url}",
                    target: "_blank",
                    rel: "noopener noreferrer",
                    class: "repo-link",
                    "View on GitHub \u{2192}"
                }
                if !project.tags.is_empty() {
                    div { class: "tag-row",
                        for tag in project.tags {
                            span { class: "tag", "{tag}" }
                        }
                    }
                }
            }

            section { class: "project-section",
                h2 { "Objective" }
                p { LinkedText { text: project.objective.to_string() } }
                ProjectGallery { items: project.media }
            }

            section { class: "project-section",
                h2 { "Approach" }
                ul {
                    for point in project.approach {
                        li { LinkedText { text: point.to_string() } }
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
                p { LinkedText { text: project.progress.to_string() } }
                p { class: "impact-statement", "{project.impact}" }
            }
        }
    }
}
