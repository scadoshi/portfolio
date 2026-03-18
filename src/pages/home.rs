use dioxus::prelude::*;

use crate::Route;
use crate::components::project_card::ProjectCard;
use crate::data;

#[component]
pub fn Home() -> Element {
    let projects = data::featured_projects();
    rsx! {
        section { class: "hero",
            h1 { class: "hero-name", "Scotty Fermo" }
            p { class: "hero-title", "Software Engineer | Full-Stack | Rust" }
            p { class: "hero-tagline",
                "I build production Rust systems that solve hard problems."
            }
            div { class: "hero-links",
                a {
                    href: "https://github.com/scadoshi",
                    target: "_blank",
                    rel: "noopener noreferrer",
                    "GitHub"
                }
                a {
                    href: "https://www.linkedin.com/in/scotty-fermo-41a35b141/",
                    target: "_blank",
                    rel: "noopener noreferrer",
                    "LinkedIn"
                }
                a {
                    href: "mailto:scottyfermo@hotmail.com",
                    "Email"
                }
            }
        }
        section { class: "about",
            p {
                "4+ years of building production systems and leading technical teams. "
                "1+ years of intensive Rust development \u{2014} from near-zero to shipping production systems. "
                "Builder mentality: when I see inefficiency, I build tools to eliminate it."
            }
        }
        section { class: "projects-section",
            h2 { class: "section-title", "Featured Projects" }
            div { class: "projects-grid",
                for project in projects {
                    ProjectCard {
                        name: project.name.to_string(),
                        slug: project.slug.to_string(),
                        category: project.category.to_string(),
                        summary: project.summary.to_string(),
                        impact_metric: project.impact_metric.to_string(),
                        impact_detail: project.impact_detail.to_string(),
                        repo_url: project.repo_url.to_string(),
                    }
                }
            }
        }
        div { class: "featured-side-quest",
            div { class: "fsq-inner",
                span { class: "fsq-label", "// featured side quest" }
                span { class: "fsq-name", "Nighthawk" }
                span { class: "fsq-blurb", "LSM-tree storage engine from scratch" }
                Link {
                    to: Route::SideQuestDetail { slug: "nighthawk".to_string() },
                    class: "fsq-link",
                    "view \u{2192}"
                }
            }
        }
    }
}
