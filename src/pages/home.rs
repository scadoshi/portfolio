use dioxus::prelude::*;
use zwipe_components::{Banner, BannerStatus};

use crate::components::page_meta::PageMeta;
use crate::components::project_card::ProjectCard;
use crate::data;
use crate::Route;

const LOGO_ASCII: &str = include_str!("../../assets/scotty.txt");

#[component]
pub fn Home() -> Element {
    let projects = data::featured_projects();
    rsx! {
        PageMeta {
            title: "Scotty Fermo",
            description: "Personal portfolio of Scotty Fermo. Production Rust systems, full-stack engineering, and side quests in protocol design, storage engines, and ML.",
            path: "/",
        }
        div { class: "banner-stack",
            Banner {
                category: "Announcement",
                status: BannerStatus::Done,
                "Zwipe, the deck builder MTG deserved. "
                a {
                    href: "https://zwipe.net",
                    target: "_blank",
                    rel: "noopener noreferrer",
                    "Try it now \u{2197}"
                }
            }
            Banner {
                category: "Featured",
                status: BannerStatus::Doing,
                "Diprotodon, a hand-written Redis-compatible KV server. "
                Link {
                    to: Route::SideQuestDetail { slug: "diprotodon".to_string() },
                    "Check it out \u{2192}"
                }
            }
        }
        section { class: "hero content-enter",
            h1 { class: "logo", "aria-label": "Scotty Fermo", "{LOGO_ASCII}" }
            div { class: "hero-panel",
                p { class: "hero-tagline",
                    "Production "
                    span { class: "hero-tagline-accent", "Rust" }
                    " for the work that has to actually run \u{2014} "
                    span { class: "hero-tagline-accent", "mobile apps" }
                    ", "
                    span { class: "hero-tagline-accent", "internal tooling" }
                    ", and the "
                    span { class: "hero-tagline-accent", "storage engines" }
                    " underneath."
                }
            }
            div { class: "hero-links",
                a {
                    href: "https://github.com/scadoshi",
                    target: "_blank",
                    rel: "noopener noreferrer",
                    "GitHub \u{2197}"
                }
                a {
                    href: "https://www.linkedin.com/in/scotty-fermo-41a35b141/",
                    target: "_blank",
                    rel: "noopener noreferrer",
                    "LinkedIn \u{2197}"
                }
                a {
                    href: "mailto:scottyfermo@hotmail.com",
                    "Email \u{2197}"
                }
            }
        }
        section { class: "about",
            p { class: "about-subtitle", "Software Engineer | Full-Stack | Rust" }
            p {
                "4+ years of building production systems and leading technical teams. "
                "1+ years of intensive Rust development \u{2014} from near-zero to shipping production systems. "
                "At Halo Software I shipped CLI tools that turned multi-week manual migrations into one-command jobs. "
                "On my own I built Zwipe (a full-stack mobile MTG deck builder, submitted to both app stores) and went deep on the storage engines and protocols underneath \u{2014} an LSM-tree KV database and a Redis-compatible server, both hand-written."
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
                        bullets: project.card_bullets.iter().map(|b| b.to_string()).collect(),
                        impact_metric: project.impact_metric.to_string(),
                        repo_url: project.repo_url.to_string(),
                        status: project.status.banner_status(),
                        status_label: project.status.label().to_string(),
                    }
                }
            }
        }
    }
}
