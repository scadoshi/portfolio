use dioxus::prelude::*;
use zwipe_components::{Banner, BannerStatus, Panel};

use crate::{
    Route,
    components::{page_meta::PageMeta, project_card::ProjectCard},
    data,
};

const LOGO_ASCII: &str = include_str!("../../assets/scotty.txt");

/// JSON-LD `Person` markup emitted into `<head>` on the home page. Ties the
/// domain to the GitHub/LinkedIn profiles for rich person results in search.
const JSON_LD: &str = r#"{
  "@context": "https://schema.org",
  "@type": "Person",
  "name": "Scotty Fermo",
  "url": "https://scottyfermo.com",
  "jobTitle": "Software Engineer",
  "email": "mailto:scottyfermo@hotmail.com",
  "sameAs": [
    "https://github.com/scadoshi",
    "https://www.linkedin.com/in/scotty-fermo-41a35b141/"
  ],
  "knowsAbout": ["Rust", "Full-stack development", "Mobile apps", "Storage engines", "Internal tooling"]
}"#;

#[component]
pub fn Home() -> Element {
    let projects = data::featured_projects();
    rsx! {
        PageMeta {
            title: "Scotty Fermo",
            description: "Personal portfolio of Scotty Fermo. Production Rust systems, full-stack engineering, and side quests in protocol design, storage engines, and ML.",
            path: "/",
        }
        document::Script { r#type: "application/ld+json", "{JSON_LD}" }
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
                    "Check it out"
                }
            }
        }
        section { class: "hero content-enter",
            h1 { class: "logo", "aria-label": "Scotty Fermo", "{LOGO_ASCII}" }
            // Sizing wrapper only; the card itself is the shared Panel.
            div { class: "hero-panel",
                Panel {
                    p { class: "hero-tagline",
                    "Production "
                    span { class: "hl-warning", "Rust" }
                    " for the work that has to actually run: "
                    span { class: "hl-success", "mobile apps" }
                    ", "
                    span { class: "hl-error", "internal tooling" }
                    ", and the "
                        span { class: "hl-tertiary", "storage engines" }
                        " underneath."
                    }
                }
            }
        }
        // One lateral band below the hero, zite-style: the about/side-quest
        // stack fills what used to be dead space beside the project cards, so
        // nothing renders as bare text on the grid and the page stays compact.
        section { class: "home-band",
            div { class: "band-col",
                h2 { class: "sr-only", "About" }
                Panel {
                    eyebrow: "About",
                    title: "Software Engineer | Full-Stack | Rust",
                    p { class: "about-text",
                        "4+ years of building production systems and leading technical teams. "
                        "2+ years of intensive Rust development, from near-zero to shipping production systems. "
                        "At Halo Software I shipped CLI tools that turned multi-week manual migrations into one-command jobs. "
                        "On my own I built Zwipe (a full-stack mobile MTG deck builder, live on both app stores) and went deep on the storage engines and protocols underneath: an LSM-tree KV database and a Redis-compatible server, both hand-written."
                    }
                }
                Panel {
                    eyebrow: "Explore",
                    title: "Side Quests",
                    actions: rsx! {
                        Link {
                            to: Route::SideQuests {},
                            class: "panel-action",
                            "View Side Quests"
                        }
                        for quest in data::side_quests() {
                            Link {
                                to: Route::SideQuestDetail { slug: quest.slug.to_string() },
                                class: "panel-action",
                                "{quest.name}"
                            }
                        }
                    },
                    p { class: "card-summary",
                        "Proofs of concept and learning projects. Each one explores a domain I wanted to understand by building something real."
                    }
                    ul { class: "card-bullets",
                        for quest in data::side_quests() {
                            li { "{quest.name}: {quest.category}" }
                        }
                    }
                }
                Panel {
                    eyebrow: "Support",
                    title: "Contribute",
                    actions: rsx! {
                        Link {
                            to: Route::Contribute {},
                            class: "panel-action",
                            "Contribute"
                        }
                        a {
                            href: crate::pages::contribute::STRIPE_URL,
                            target: "_blank",
                            rel: "noopener noreferrer",
                            class: "panel-action",
                            "Stripe \u{2197}"
                        }
                        a {
                            href: crate::pages::contribute::BMC_URL,
                            target: "_blank",
                            rel: "noopener noreferrer",
                            class: "panel-action",
                            "Buy Me a Coffee \u{2197}"
                        }
                        a {
                            href: crate::pages::contribute::GITHUB_SPONSORS_URL,
                            target: "_blank",
                            rel: "noopener noreferrer",
                            class: "panel-action",
                            "GitHub Sponsors \u{2197}"
                        }
                    },
                    p { class: "card-summary",
                        "I build open-source Rust tools. If my work has been useful, consider supporting continued development."
                    }
                }
            }
            div { class: "band-col",
                h2 { class: "sr-only", "Featured Projects" }
                for project in projects {
                    ProjectCard {
                        name: project.name.to_string(),
                        slug: project.slug.to_string(),
                        category: project.category.to_string(),
                        summary: project.summary.to_string(),
                        bullets: project.card_bullets.iter().map(|b| b.to_string()).collect(),
                        impact_metric: project.impact_metric.to_string(),
                        repo_url: project.repo_url.to_string(),
                        site_url: project.site_url.map(str::to_string),
                        status: project.status.banner_status(),
                        status_label: project.status.label().to_string(),
                    }
                }
            }
        }
    }
}
