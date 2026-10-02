use dioxus::prelude::*;
use zwipe_components::Panel;

use crate::{
    Route,
    components::{
        benchmark::Benchmark, code_block::CodeBlock, commits::Commits, fleet::Fleet, flow::Flow,
        gallery::ProjectGallery, linked_text::LinkedText, measured::Measured, page_meta::PageMeta,
    },
    data,
};

/// Kept as a function rather than inlined so the panel's shape stays readable
/// next to the rest of the page's sections.
fn approach_panel(project: &'static data::Project) -> Element {
    rsx! {
        Panel {
            eyebrow: "Approach",
            title: "How it's built",
            section { class: "project-section",
                ul {
                    for point in project.approach {
                        li { LinkedText { text: point.to_string() } }
                    }
                }
            }
        }
    }
}

/// Panels only one project has. heron is the project this site is a client of,
/// so its page shows the path a number takes and what heron is measuring right
/// now; steller's page shows its benchmark against Redis. Empty for the rest.
fn project_panels(project: &'static data::Project) -> Element {
    match project.slug {
        // The numbers first, then the path they took to get here.
        "heron" => rsx! {
            Panel {
                eyebrow: "Measured",
                title: "What it is counting right now",
                section { class: "project-section",
                    Fleet {}
                }
            }
            Panel {
                eyebrow: "Over time",
                title: "Commits, month by month",
                section { class: "project-section",
                    Commits {}
                }
            }
            Panel {
                eyebrow: "Pipeline",
                title: "How a number gets here",
                section { class: "project-section",
                    Flow {}
                }
            }
        },
        "steller" => rsx! {
            Panel {
                eyebrow: "Measured",
                title: "Against Redis 8",
                section { class: "project-section",
                    Benchmark {}
                }
            }
        },
        _ => rsx! {},
    }
}

/// Shared body for both detail pages. Projects and side quests render
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
            // Identity, headline, and tags in one card so no text sits
            // directly on the grid.
            Panel {
                eyebrow: project.category.to_string(),
                status: project.status.banner_status(),
                status_label: project.status.label().to_string(),
                title: project.name.to_string(),
                // Page hero: the shared Panel renders its title as an h3 by
                // default (right for cards, wrong for a page's main heading).
                title_h1: true,
                // The content tags sit in the heading and wrap with it, which
                // keeps them apart from the measured chips below.
                title_trailing: rsx! {
                    for (i, tag) in project.tags.iter().enumerate() {
                        span { class: "tag title-tag tag-c{i % 4}", "{tag}" }
                    }
                },
                actions: rsx! {
                    a {
                        href: "{project.repo_url}",
                        class: "panel-action",
                        "View on GitHub" span { class: "ext", "\u{2197}" }
                    }
                    if let Some(site) = project.site_url {
                        a {
                            href: "{site}",
                            class: "panel-action",
                            {site.trim_start_matches("https://")}
                            span { class: "ext", "\u{2197}" }
                        }
                    }
                },
                p { class: "project-headline", "{project.headline}" }
                Measured { repo_url: project.repo_url.to_string() }
            }

            Panel {
                eyebrow: "Objective",
                title: "The goal",
                section { class: "project-section",
                    p { LinkedText { text: project.objective.to_string() } }
                }
            }

            {project_panels(project)}

            // Full width on its own: pairing it with the approach panel leaves
            // the gallery above dead space, and the media is 16:9 anyway.
            if !project.media.is_empty() {
                ProjectGallery { items: project.media }
            }

            {approach_panel(project)}

            // Code wants the full column width, so implementation stays a
            // single wide panel.
            Panel {
                eyebrow: "Implementation",
                title: "The code up close",
                section { class: "project-section",
                    for snippet in project.snippets {
                        CodeBlock {
                            key: "{project.slug}-{snippet.title}",
                            title: snippet.title.to_string(),
                            code: snippet.code.to_string(),
                            description: snippet.description.to_string(),
                            lang: snippet.lang.to_string(),
                        }
                    }
                }
            }

            div { class: "detail-band",
                Panel {
                    eyebrow: "Obstacles",
                    title: "What fought back",
                    section { class: "project-section",
                        ul {
                            for obstacle in project.obstacles {
                                li { LinkedText { text: obstacle.to_string() } }
                            }
                        }
                    }
                }
                Panel {
                    eyebrow: "Progress & Impact",
                    title: "Where it stands",
                    section { class: "project-section",
                        p { LinkedText { text: project.progress.to_string() } }
                        p { class: "impact-statement", "{project.impact}" }
                    }
                }
            }
        }
    }
}

/// A page that has moved.
///
/// Prerendered with a refresh and a canonical, which is what a direct visit and a
/// crawler see. A visit from inside the app never loads that HTML, so the route is
/// replaced here as well.
#[component]
fn Moved(to: &'static str) -> Element {
    use_effect(move || {
        if let Ok(route) = to.parse::<Route>() {
            navigator().replace(route);
        }
    });
    rsx! {
        PageMeta {
            title: "Moved",
            description: "This page is now at {to}.",
            path: to,
        }
        document::Meta { http_equiv: "refresh", content: "0; url={to}" }
        div { class: "not-found",
            h1 { "Moved" }
            p {
                "This page is now at "
                a { href: "{to}", "{to}" }
                "."
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

/// Featured project page at `/projects/:slug`.
#[component]
pub fn ProjectDetail(slug: String) -> Element {
    if let Some(to) = data::moved_to(&format!("/projects/{slug}")) {
        return rsx! { Moved { to } };
    }
    let Some(project) = data::find_project(&slug) else {
        return not_found("Project", &slug);
    };
    detail_view(project, format!("/projects/{}", project.slug))
}

/// Side quest page at `/side-quests/:slug`.
#[component]
pub fn SideQuestDetail(slug: String) -> Element {
    if let Some(to) = data::moved_to(&format!("/side-quests/{slug}")) {
        return rsx! { Moved { to } };
    }
    let Some(project) = data::find_side_quest(&slug) else {
        return not_found("Side quest", &slug);
    };
    detail_view(project, format!("/side-quests/{}", project.slug))
}
