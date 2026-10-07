use dioxus::prelude::*;
use zwipe_components::{Decode, Panel, Replay, StatsStrip};

use crate::{
    Route,
    components::{heatmap::Heatmap, page_meta::PageMeta, project_card::project_card},
    data::{self, Section},
    stats,
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
  "knowsAbout": ["Rust", "Full-stack development", "Mobile apps", "Servers", "Key-value stores"]
}"#;

#[component]
pub fn Home() -> Element {
    // Featured products go in the left column under About; the rest, the
    // systems work, fill the right.
    let (products, others): (Vec<&'static data::Project>, Vec<&'static data::Project>) =
        data::featured()
            .iter()
            .partition(|p| data::section_of(p.slug) == Some(Section::Products));
    let live = use_context::<stats::Live>();
    let totals = stats::totals(live.read().as_ref());
    let replay = use_context::<Replay>().0();
    rsx! {
        // Title lands at 60 chars with PageMeta's " | Scotty Fermo" suffix;
        // description stays under the ~125-char social-preview cutoff.
        PageMeta {
            title: "Software Engineer: Rust, Full-Stack & Systems",
            description: "Scotty Fermo, software engineer. Rust that runs: a mobile app on both stores, a live server, two key-value stores.",
            path: "/",
        }
        document::Script { r#type: "application/ld+json", "{JSON_LD}" }
        section { class: "hero content-enter",
            // Keyed on the replay count so the spacing animation runs again
            // with the decode.
            // Sizing wrapper only; the card itself is the shared Panel.
            div { class: "hero-panel",
                Panel {
                    // The name, then the numbers it stands for, then the
                    // sentence under both.
                    div { class: "hero-head",
                        for run in [replay] {
                            h1 { key: "logo{run}", class: "logo", "aria-label": "Scotty Fermo", Decode { text: LOGO_ASCII } }
                        }
                        if let Some((totals, source)) = totals.as_ref() {
                            StatsStrip {
                                figures: vec![
                                    (Some(totals.commits), "Commits"),
                                    (Some(u64::from(totals.repos)), "Repos"),
                                    (Some(totals.stars), "Stars"),
                                ],
                                // Where the numbers come from, as chips under the strip.
                                source: rsx! {
                                    Link {
                                        class: "tag tag-c0",
                                        to: Route::ProjectDetail { slug: "heron".to_string() },
                                        "counted by heron"
                                    }
                                    Link {
                                        class: "tag tag-c0",
                                        to: Route::ProjectDetail { slug: "steller".to_string() },
                                        "cached in steller"
                                    }
                                    // Keyed on its text, so the live chip arrives with the
                                    // ease rather than the as-of chip changing its words.
                                    {
                                        let label = match source {
                                            stats::Source::Live => "live".to_string(),
                                            stats::Source::AsOf(day) => format!("as of {day}"),
                                        };
                                        rsx! { span { key: "{label}", class: "tag tag-c0 tag-swap", "{label}" } }
                                    }
                                },
                            }
                        }
                    }
                    p { class: "hero-tagline",
                        "Software engineer at Halo Software, writing production C#. On my own time, "
                        span { class: "hl-warning", "Rust" }
                        " that runs: an "
                        span { class: "hl-success", "app" }
                        " on both stores, a live "
                        span { class: "hl-error", "server" }
                        ", and two "
                        span { class: "hl-tertiary", "key-value stores" }
                        " built to learn how the real ones work."
                    }
                    // The year of contributions, from the same answer.
                    if totals.is_some() {
                        hr { class: "hero-rule" }
                        Heatmap {}
                    }
                }
            }
        }
        // One lateral band below the hero, zite-style: the about/sections
        // stack sits beside the project cards so nothing renders as bare text
        // on the grid and the page stays compact.
        section { class: "home-band",
            div { class: "band-col band-aside",
                h2 { class: "sr-only", "About" }
                Panel {
                    eyebrow: "About",
                    title: "Software Engineer | Systems | Rust",
                    p { class: "about-text",
                        "The day job is a large, mature enterprise codebase. "
                        "Before moving into the core product, I spent four years designing request-management systems for universities, government bodies and financial institutions, and wrote the CLI tools that turned multi-week data migrations into one-command jobs. "
                        "The Rust is where I go deeper. Both key-value stores went past the tutorial: steller is benchmarked against Redis and serves a live service, and both were debugged where real clients broke them. Zwipe is the proof I can ship a whole product alone. "
                        "What I enjoy most is what's on this page: storage, wire protocols, the parts of a system that have to be right."
                    }
                }
                // Featured products sit under About, apart from the systems
                // cards on the right; display: contents, so each card is a
                // panel of the column (main.css).
                div { class: "band-featured",
                    for project in products {
                        {project_card(project)}
                    }
                }
                Panel {
                    eyebrow: "Process",
                    title: "How I Use AI",
                    p { class: "card-summary",
                        "Slow on purpose while learning. Fast once I understand it."
                    }
                    ul { class: "card-bullets",
                        li { "Learning projects I write by hand; the reps are the point" }
                        li { "Once I can explain every line, AI speeds up the iteration and refactoring" }
                        li { "I read and test all of it, and all of it is open source" }
                    }
                }
                Panel {
                    eyebrow: "Explore",
                    title: "All Projects",
                    actions: rsx! {
                        Link {
                            to: Route::Projects {},
                            class: "panel-action",
                            "View All Projects"
                        }
                        for section in Section::ALL {
                            Link {
                                to: Route::SectionPage { section },
                                class: "panel-action",
                                "{section.name()}"
                            }
                        }
                    },
                    p { class: "card-summary",
                        "The cards here are a pick. Everything else is in four sections."
                    }
                    ul { class: "card-bullets",
                        for section in Section::ALL {
                            li {
                                "{section.name()}: "
                                {section.projects().iter().map(|p| p.name).collect::<Vec<_>>().join(", ")}
                            }
                        }
                    }
                }
            }
            div { class: "band-col band-main",
                h2 { class: "sr-only", "Featured Projects" }
                for project in others {
                    {project_card(project)}
                }
            }
        }
    }
}
