use dioxus::prelude::*;
use zwipe_components::{BannerStatus, Panel};

use crate::{Route, counts, stats};

#[component]
pub fn ProjectCard(
    name: String,
    slug: String,
    category: String,
    summary: String,
    bullets: Vec<String>,
    impact_metric: String,
    repo_url: String,
    #[props(default)] site_url: Option<String>,
    status: BannerStatus,
    status_label: String,
) -> Element {
    let live = use_context::<stats::Live>();
    let commit_line = stats::line_for(live.read().as_ref(), &repo_url);
    let counts_line = counts::line_for(&repo_url);
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
                    class: "panel-action",
                    "GitHub" span { class: "ext", "\u{2197}" }
                }
                if let Some(site) = site_url {
                    a {
                        href: "{site}",
                        class: "panel-action",
                        {site.trim_start_matches("https://")}
                        span { class: "ext", "\u{2197}" }
                    }
                }
            },
            p { class: "card-summary", "{summary}" }
            ul { class: "card-bullets",
                for bullet in bullets {
                    li { "{bullet}" }
                }
            }
            div { class: "card-impact", "{impact_metric}" }
            if let Some(commit_line) = commit_line {
                p { class: "card-stats", "{commit_line}" }
            }
            if let Some(counts_line) = counts_line {
                p { class: "card-stats", "{counts_line}" }
            }
        }
    }
}
