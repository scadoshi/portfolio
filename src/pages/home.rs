use dioxus::prelude::*;

use crate::components::project_card::ProjectCard;
use crate::data;
use crate::Route;

const LOGO_ASCII: &str = include_str!("../../assets/scotty.txt");

#[component]
pub fn Home() -> Element {
    let projects = data::featured_projects();
    let mut zwipe_banner = use_signal(|| true);
    let mut dipro_banner = use_signal(|| true);
    rsx! {
        if zwipe_banner() || dipro_banner() {
            div { class: "banner-stack",
                if zwipe_banner() {
                    div { class: "announcement-banner",
                        div { class: "banner-header",
                            span { class: "banner-category", "Announcement" }
                            span { class: "status-tag status-done", "Live" }
                        }
                        span { class: "banner-text",
                            "Zwipe, the deck builder MTG deserved. "
                            a {
                                href: "https://zwipe.net",
                                target: "_blank",
                                rel: "noopener noreferrer",
                                "Try it now \u{2192}"
                            }
                        }
                        button {
                            class: "banner-dismiss",
                            onclick: move |_| zwipe_banner.set(false),
                            "\u{2715}"
                        }
                        div {
                            class: "banner-progress",
                            onanimationend: move |_| zwipe_banner.set(false),
                        }
                    }
                }
                if dipro_banner() {
                    div { class: "announcement-banner",
                        div { class: "banner-header",
                            span { class: "banner-category", "Featured" }
                            span { class: "status-tag status-doing", "In Progress" }
                        }
                        span { class: "banner-text",
                            "Diprotodon, a hand-written Redis-compatible KV server. "
                            Link {
                                to: Route::SideQuestDetail { slug: "diprotodon".to_string() },
                                "Check it out \u{2192}"
                            }
                        }
                        button {
                            class: "banner-dismiss",
                            onclick: move |_| dipro_banner.set(false),
                            "\u{2715}"
                        }
                        div {
                            class: "banner-progress",
                            onanimationend: move |_| dipro_banner.set(false),
                        }
                    }
                }
            }
        }
        section { class: "hero content-enter",
            div { class: "logo", "{LOGO_ASCII}" }
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
                        bullets: project.card_bullets.iter().map(|b| b.to_string()).collect(),
                        impact_metric: project.impact_metric.to_string(),
                        repo_url: project.repo_url.to_string(),
                        status_label: project.status.label().to_string(),
                        status_class: project.status.css_class().to_string(),
                    }
                }
            }
        }
        div { class: "featured-side-quest",
            div { class: "fsq-inner",
                span { class: "fsq-label", "// btw" }
                span { class: "fsq-name", "this site is built in Rust" }
                span { class: "fsq-blurb", "Dioxus + WASM, no JS" }
                a {
                    href: "https://github.com/scadoshi/portfolio",
                    target: "_blank",
                    rel: "noopener noreferrer",
                    class: "fsq-link",
                    "view repo \u{2192}"
                }
            }
        }
    }
}
