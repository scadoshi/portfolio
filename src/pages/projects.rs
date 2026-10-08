use dioxus::prelude::*;
use zwipe_components::Panel;

use crate::{
    Route,
    components::{page_meta::PageMeta, project_card::project_card},
    data::Section,
};

/// The overview: one short panel per section, naming its projects. The cards
/// themselves live on each section's own page, so this one stays short.
#[component]
pub fn Projects() -> Element {
    rsx! {
        PageMeta {
            title: "Projects",
            description: "Every project, in four sections: finished products, learning builds of backend systems, work tooling, and experiments.",
            path: "/projects",
        }
        div { class: "index-page content-enter",
            Panel {
                title: "Projects",
                // Page hero, so the title is this page's h1 (see detail.rs).
                title_h1: true,
                p { class: "card-summary",
                    "Everything worth reading about, in four sections: what I've shipped, how I learn, and what I've built for work."
                }
            }
            div { class: "projects-grid",
                for section in Section::ALL {
                    Panel {
                        key: "{section.slug()}",
                        title: section.name().to_string(),
                        actions: rsx! {
                            Link {
                                to: Route::SectionPage { section },
                                class: "panel-action",
                                "View {section.name()}"
                            }
                        },
                        p { class: "card-summary", "{section.blurb()}" }
                        ul { class: "card-bullets",
                            for project in section.projects() {
                                li { key: "{project.slug}",
                                    Link {
                                        to: Route::ProjectDetail { slug: project.slug.to_string() },
                                        "{project.name}"
                                    }
                                    ": {project.category}"
                                }
                            }
                        }
                    }
                }
            }
        }
    }
}

/// One section's cards at `/projects/<section>`.
#[component]
pub fn SectionPage(section: Section) -> Element {
    rsx! {
        PageMeta {
            title: section.name().to_string(),
            description: section.blurb().to_string(),
            path: "/projects/{section}",
        }
        div { class: "index-page content-enter",
            Panel {
                title: section.name().to_string(),
                // Page hero, so the title is this page's h1 (see detail.rs).
                title_h1: true,
                actions: rsx! {
                    Link { to: Route::Projects {}, class: "panel-action", "All Sections" }
                    // The other three, so moving between shelves is one click.
                    for other in Section::ALL.into_iter().filter(|other| *other != section) {
                        Link {
                            key: "{other.slug()}",
                            to: Route::SectionPage { section: other },
                            class: "panel-action",
                            "{other.name()}"
                        }
                    }
                },
                p { class: "card-summary", "{section.blurb()}" }
            }
            div { class: "projects-grid",
                for project in section.projects() {
                    {project_card(project)}
                }
            }
        }
    }
}
