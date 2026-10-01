use dioxus::prelude::*;
use zwipe_components::{Banner, Panel};

use crate::{
    Route,
    components::{heatmap::Heatmap, page_meta::PageMeta, project_card::ProjectCard},
    data, stats,
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
  "knowsAbout": ["Rust", "Full-stack development", "Mobile apps", "Servers", "Storage engines"]
}"#;

#[component]
pub fn Home() -> Element {
    let projects = data::featured_projects();
    let live = use_context::<stats::Live>();
    let totals = stats::totals(live.read().as_ref());
    rsx! {
        // Title lands at 60 chars with PageMeta's " | Scotty Fermo" suffix;
        // description stays under the ~125-char social-preview cutoff.
        PageMeta {
            title: "Software Engineer: Rust, Full-Stack & Systems",
            description: "Scotty Fermo, software engineer. Rust that runs: a mobile app on both stores, a live server, two storage engines.",
            path: "/",
        }
        document::Script { r#type: "application/ld+json", "{JSON_LD}" }
        div { class: "banner-stack",
            Banner {
                "The numbers on this page come from heron, my server, cached in steller, my Redis. "
                Link {
                    to: Route::SideQuestDetail { slug: "heron".to_string() },
                    "How it works"
                }
            }
        }
        section { class: "hero content-enter",
            h1 { class: "logo", "aria-label": "Scotty Fermo", "{LOGO_ASCII}" }
            // Sizing wrapper only; the card itself is the shared Panel.
            div { class: "hero-panel",
                Panel {
                    p { class: "hero-tagline",
                        span { class: "hl-warning", "Rust" }
                        " that runs: an "
                        span { class: "hl-success", "app" }
                        " on both stores, a "
                        span { class: "hl-error", "server" }
                        " counting the numbers above, and two "
                        span { class: "hl-tertiary", "storage engines" }
                        " written from the wire up."
                    }
                    // Same strip as zwipe.net's hero: a rule, then the numbers.
                    // A second rule sets off where they come from.
                    if let Some((totals, source)) = totals {
                        hr { class: "hero-rule" }
                        section { class: "stats-strip",
                            div { class: "stat",
                                span { class: "stat-num", {stats::with_separators(totals.commits)} }
                                span { class: "stat-label", "Commits" }
                            }
                            div { class: "stat",
                                span { class: "stat-num", "{totals.repos}" }
                                span { class: "stat-label", "Repos" }
                            }
                            div { class: "stat",
                                span { class: "stat-num", {stats::with_separators(totals.stars)} }
                                span { class: "stat-label", "Stars" }
                            }
                        }
                        // Where the numbers come from, as chips under the strip.
                        div { class: "tag-row stats-source",
                            Link {
                                class: "tag tag-c0",
                                to: Route::SideQuestDetail { slug: "heron".to_string() },
                                "counted by heron"
                            }
                            Link {
                                class: "tag tag-c0",
                                to: Route::ProjectDetail { slug: "steller".to_string() },
                                "cached in steller"
                            }
                            match source {
                                stats::Source::Live => rsx! { span { class: "tag tag-c0", "live" } },
                                stats::Source::AsOf(day) => rsx! { span { class: "tag tag-c0", "as of {day}" } },
                            }
                        }
                        // The year of contributions, from the same answer.
                        hr { class: "hero-rule" }
                        Heatmap {}
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
                        "I write production C# into Halo Software's core product, a large, mature enterprise codebase, shipping through the same pipeline their staff engineers use. "
                        "Before that, four years at Halo designing enterprise request-management systems for universities, government entities, and financial institutions, and building the CLI tools that turned multi-week migrations into one-command jobs. "
                        "My own work is Rust and systems: a hand-written LSM-tree storage engine and a Redis-compatible server. Zwipe, a full-stack mobile app live on both stores, is the proof I can ship the whole thing alone. "
                        "The work I want more of is what's on this page: storage, wire protocols, the parts of a system that have to be right."
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
