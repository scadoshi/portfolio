use dioxus::prelude::*;
use zwipe_components::Panel;

use crate::components::page_meta::PageMeta;

const STRIPE_URL: &str = "https://buy.stripe.com/5kQdRa5tUeNm9pd8BY9Zm00";
const BMC_URL: &str = "https://buymeacoffee.com/scadoshi";
const GITHUB_SPONSORS_URL: &str = "https://github.com/sponsors/scadoshi";

#[component]
pub fn Contribute() -> Element {
    rsx! {
        PageMeta {
            title: "Contribute",
            description: "Support continued open-source Rust development from Scotty Fermo via Stripe, Buy Me a Coffee, or GitHub Sponsors.",
            path: "/contribute",
        }
        div { class: "side-quests content-enter",
            section { class: "page-header",
                h1 { "Contribute" }
                p { class: "page-subtitle",
                    "I build open-source Rust tools. If my work has been useful, consider supporting continued development."
                }
            }
            div { class: "projects-grid",
                Panel {
                    eyebrow: "One-Time",
                    title: "Stripe",
                    actions: rsx! {
                        a {
                            href: STRIPE_URL,
                            target: "_blank",
                            rel: "noopener noreferrer",
                            class: "panel-action",
                            "Contribute \u{2197}"
                        }
                    },
                    p { class: "card-summary", "Pay what you want. No account required." }
                }
                Panel {
                    eyebrow: "One-Time",
                    title: "Buy Me a Coffee",
                    actions: rsx! {
                        a {
                            href: BMC_URL,
                            target: "_blank",
                            rel: "noopener noreferrer",
                            class: "panel-action",
                            "buymeacoffee.com/scadoshi \u{2197}"
                        }
                    },
                    p { class: "card-summary", "Quick one-off support through Buy Me a Coffee." }
                }
                Panel {
                    eyebrow: "Recurring",
                    title: "GitHub Sponsors",
                    actions: rsx! {
                        a {
                            href: GITHUB_SPONSORS_URL,
                            target: "_blank",
                            rel: "noopener noreferrer",
                            class: "panel-action",
                            "github.com/sponsors/scadoshi \u{2197}"
                        }
                    },
                    p { class: "card-summary", "Recurring monthly support via GitHub Sponsors." }
                }
            }
        }
    }
}
