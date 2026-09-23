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
  "jobTitle": "Software Developer",
  "worksFor": { "@type": "Organization", "name": "Halo Software" },
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
        // Title lands at 60 chars with PageMeta's " | Scotty Fermo" suffix;
        // description stays under the ~125-char social-preview cutoff.
        PageMeta {
            title: "Software Engineer: Rust, Full-Stack & Systems",
            description: "Personal portfolio of Scotty Fermo: production Rust systems, full-stack engineering, storage engines, and ML side quests.",
            path: "/",
        }
        document::Script { r#type: "application/ld+json", "{JSON_LD}" }
        div { class: "banner-stack",
            Banner {
                category: "Announcement",
                status: BannerStatus::Done,
                status_label: "Live",
                "Zwipe, the deck builder MTG deserved. "
                a {
                    href: "https://zwipe.net",
                    "Try it now" span { class: "ext", "\u{2197}" }
                }
            }
            Banner {
                category: "Featured",
                status: BannerStatus::Done,
                "Steller, a hand-written Redis-compatible KV server. "
                Link {
                    to: Route::SideQuestDetail { slug: "steller".to_string() },
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
                        span { class: "hl-success", "storage engines" }
                        ", "
                        span { class: "hl-error", "wire protocols" }
                        ", and the "
                        span { class: "hl-tertiary", "systems" }
                        " underneath."
                    }
                }
            }
        }
        // One lateral band below the hero, zite-style: the about/side-quest
        // stack sits beside the project cards so nothing renders as bare text
        // on the grid and the page stays compact.
        section { class: "home-band",
            div { class: "band-col band-aside",
                h2 { class: "sr-only", "About" }
                Panel {
                    eyebrow: "About",
                    title: "Software Engineer | Systems | Rust",
                    p { class: "about-text",
                        "I write production C# into Halo Software's core product, a large legacy enterprise codebase, shipping through the same pipeline their staff engineers use. "
                        "Four years there before that designing enterprise request-management systems for universities, government entities, and financial institutions, and building the CLI tools that turned multi-week migrations into one-command jobs. "
                        "My own work is Rust and systems: a hand-written LSM-tree storage engine and a Redis-compatible server. Zwipe, a full-stack mobile app live on both stores, is the proof I can ship the whole thing alone."
                    }
                }
                Panel {
                    eyebrow: "Process",
                    title: "How I Use AI",
                    p { class: "card-summary",
                        "Slow on purpose while learning. Fast once I understand it."
                    }
                    ul { class: "card-bullets",
                        li { "New domains start with my own research, cross-referenced with a model, until I can pick an approach and defend it" }
                        li { "Learning projects I write by hand, taking a beat to rely on my own brain instead of passing the mental cycles to a model. The reps are the point" }
                        li { "Once I can explain and type every line myself, AI takes over the iteration and refactoring. That's where the speed comes from" }
                        li { "Either way I read all of it: security checked at every stop, implementations validated against other models, everything tested" }
                        li { "All of it is open source. Read the code; criticism is welcome" }
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
                            class: "panel-action",
                            "Stripe" span { class: "ext", "\u{2197}" }
                        }
                        a {
                            href: crate::pages::contribute::BMC_URL,
                            class: "panel-action",
                            "Buy Me a Coffee" span { class: "ext", "\u{2197}" }
                        }
                        a {
                            href: crate::pages::contribute::GITHUB_SPONSORS_URL,
                            class: "panel-action",
                            "GitHub Sponsors" span { class: "ext", "\u{2197}" }
                        }
                    },
                    p { class: "card-summary",
                        "I build open-source Rust tools. If my work has been useful, consider supporting continued development."
                    }
                }
            }
            div { class: "band-col band-main",
                h2 { class: "sr-only", "Featured Projects" }
                for project in projects {
                    ProjectCard {
                        name: project.name.to_string(),
                        slug: project.slug.to_string(),
                        category: project.category.to_string(),
                        summary: project.summary.to_string(),
                        bullets: project.card_bullets.iter().map(std::string::ToString::to_string).collect(),
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
