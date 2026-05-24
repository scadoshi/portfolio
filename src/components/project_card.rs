use dioxus::prelude::*;

use crate::Route;

#[component]
pub fn ProjectCard(
    name: String,
    slug: String,
    category: String,
    summary: String,
    bullets: Vec<String>,
    impact_metric: String,
    repo_url: String,
    status_label: String,
    status_class: String,
) -> Element {
    rsx! {
        div { class: "project-card",
            div { class: "card-category",
                "{category}"
                span { class: "status-tag {status_class}", "{status_label}" }
            }
            h3 { class: "card-title", "{name}" }
            p { class: "card-summary", "{summary}" }
            ul { class: "card-bullets",
                for bullet in bullets {
                    li { "{bullet}" }
                }
            }
            div { class: "card-impact", "{impact_metric}" }
            div { class: "card-actions",
                Link {
                    to: Route::ProjectDetail { slug },
                    class: "card-link",
                    "View Project \u{2192}"
                }
                a {
                    href: "{repo_url}",
                    target: "_blank",
                    rel: "noopener noreferrer",
                    class: "card-link card-link-secondary",
                    "GitHub \u{2192}"
                }
            }
        }
    }
}
