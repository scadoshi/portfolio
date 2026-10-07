use dioxus::prelude::*;
use zwipe_components::Panel;

use crate::{
    components::{page_meta::PageMeta, project_card::project_card},
    data::Section,
};

/// Every project, section by section. The nav's dropdowns land on the
/// sections here by id.
#[component]
pub fn Projects() -> Element {
    rsx! {
        PageMeta {
            title: "Projects",
            description: "Every project, on four shelves: products people use, learning builds of backend systems, work tooling, and experiments.",
            path: "/projects",
        }
        div { class: "index-page content-enter",
            Panel {
                eyebrow: "Index",
                title: "Projects",
                // Page hero, so the title is this page's h1 (see detail.rs).
                title_h1: true,
                actions: rsx! {
                    for section in Section::ALL {
                        a {
                            href: "#{section.slug()}",
                            class: "panel-action",
                            "{section.name()}"
                        }
                    }
                },
                p { class: "card-summary",
                    "Everything worth reading about, on four shelves. Products are the things people use; the rest is how I learn and what I build at work."
                }
            }
            for section in Section::ALL {
                section { key: "{section.slug()}", id: section.slug(), class: "index-section",
                    h2 { class: "sr-only", "{section.name()}" }
                    Panel {
                        eyebrow: "Section",
                        title: section.name().to_string(),
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
    }
}
