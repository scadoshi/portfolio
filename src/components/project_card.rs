use dioxus::prelude::*;
use zwipe_components::{BannerStatus, Panel};

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
    status: BannerStatus,
    status_label: String,
) -> Element {
    rsx! {
        Panel {
            eyebrow: category,
            title: name,
            status,
            status_label,
            actions: rsx! {
                Link {
                    to: Route::ProjectDetail { slug },
                    class: "panel-action",
                    "View Project"
                }
                a {
                    href: "{repo_url}",
                    target: "_blank",
                    rel: "noopener noreferrer",
                    class: "panel-action",
                    "GitHub \u{2197}"
                }
            },
            p { class: "card-summary", "{summary}" }
            ul { class: "card-bullets",
                for bullet in bullets {
                    li { "{bullet}" }
                }
            }
            div { class: "card-impact", "{impact_metric}" }
        }
    }
}
