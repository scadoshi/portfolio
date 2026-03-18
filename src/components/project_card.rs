use dioxus::prelude::*;

use crate::Route;

#[component]
pub fn ProjectCard(
    name: String,
    slug: String,
    category: String,
    summary: String,
    impact_metric: String,
    impact_detail: String,
    repo_url: String,
) -> Element {
    rsx! {
        div { class: "project-card",
            div { class: "card-category", "{category}" }
            h3 { class: "card-title", "{name}" }
            p { class: "card-summary", "{summary}" }
            div { class: "card-impact",
                "{impact_metric}"
                if !impact_detail.is_empty() {
                    div { class: "card-impact-tooltip", "{impact_detail}" }
                }
            }
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
