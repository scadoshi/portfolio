use dioxus::prelude::*;
use zwipe_components::Panel;

use crate::{
    components::{
        code_block::CodeBlock, gallery::ProjectGallery, linked_text::LinkedText,
        page_meta::PageMeta,
    },
    data,
};

/// Shared body for both detail pages — projects and side quests render
/// identically off the same `Project` shape; only the lookup, canonical path,
/// and not-found wording differ (see the two components below).
fn detail_view(project: &'static data::Project, path: String) -> Element {
    rsx! {
        PageMeta {
            title: project.name.to_string(),
            description: project.headline.to_string(),
            path,
        }
        div { class: "project-detail content-enter",
            // Header panel: identity, headline, and tags in one card so no
            // text sits directly on the grid.
            Panel {
                actions: rsx! {
                    a {
                        href: "{project.repo_url}",
                        target: "_blank",
                        rel: "noopener noreferrer",
                        class: "panel-action",
                        "View on GitHub \u{2197}"
                    }
                },
                span { class: "project-category-tag",
                    "{project.category}"
                    span { class: "status-tag {project.status.css_class()}", "{project.status.label()}" }
                }
                h1 { class: "project-name", "{project.name}" }
                p { class: "project-headline", "{project.headline}" }
                if !project.tags.is_empty() {
                    div { class: "tag-row",
                        for tag in project.tags {
                            span { class: "tag", "{tag}" }
                        }
                    }
                }
            }

            Panel {
                section { class: "project-section",
                    h2 { "Objective" }
                    p { LinkedText { text: project.objective.to_string() } }
                }
            }

            // Lateral band: the demo gallery keeps one column while the
            // approach panel fills what would be its dead side-space.
            div { class: "detail-band",
                div { class: "band-col",
                    ProjectGallery { items: project.media }
                }
                div { class: "band-col",
                    Panel {
                        section { class: "project-section",
                            h2 { "Approach" }
                            ul {
                                for point in project.approach {
                                    li { LinkedText { text: point.to_string() } }
                                }
                            }
                        }
                    }
                }
            }

            // Code wants the full column width, so implementation stays a
            // single wide panel.
            Panel {
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
            }

            div { class: "detail-band",
                Panel {
                    section { class: "project-section",
                        h2 { "Obstacles" }
                        ul {
                            for obstacle in project.obstacles {
                                li { LinkedText { text: obstacle.to_string() } }
                            }
                        }
                    }
                }
                Panel {
                    section { class: "project-section",
                        h2 { "Progress & Impact" }
                        p { LinkedText { text: project.progress.to_string() } }
                        p { class: "impact-statement", "{project.impact}" }
                    }
                }
            }
        }
    }
}

/// Not-found fallback shared by both routes. `kind` is Title-case ("Project" /
/// "Side quest"); the body sentence lowercases it.
fn not_found(kind: &str, slug: &str) -> Element {
    let lower = kind.to_lowercase();
    rsx! {
        document::Title { "{kind} not found | Scotty Fermo" }
        div { class: "not-found",
            h1 { "{kind} not found" }
            p { "No {lower} matches \"{slug}\"." }
        }
    }
}

/// Featured project detail page (`/projects/:slug`).
#[component]
pub fn ProjectDetail(slug: String) -> Element {
    let Some(project) = data::find_project(&slug) else {
        return not_found("Project", &slug);
    };
    detail_view(project, format!("/projects/{}", project.slug))
}

/// Side quest detail page (`/side-quests/:slug`).
#[component]
pub fn SideQuestDetail(slug: String) -> Element {
    let Some(project) = data::find_side_quest(&slug) else {
        return not_found("Side quest", &slug);
    };
    detail_view(project, format!("/side-quests/{}", project.slug))
}
