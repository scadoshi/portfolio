use dioxus::prelude::*;

const STRIPE_URL: &str = "https://buy.stripe.com/5kQdRa5tUeNm9pd8BY9Zm00";
const BMC_URL: &str = "https://buymeacoffee.com/scadoshi";
const GITHUB_SPONSORS_URL: &str = "https://github.com/sponsors/scadoshi";

#[component]
pub fn Contribute() -> Element {
    rsx! {
        div { class: "side-quests content-enter",
            section { class: "page-header",
                h1 { "Contribute" }
                p { class: "page-subtitle",
                    "I build open-source Rust tools. If my work has been useful, consider supporting continued development."
                }
            }
            div { class: "projects-grid",
                div { class: "project-card",
                    div { class: "card-category", "One-Time" }
                    h3 { class: "card-title", "Stripe" }
                    p { class: "card-summary", "Pay what you want. No account required." }
                    div { class: "card-actions",
                        a {
                            href: STRIPE_URL,
                            target: "_blank",
                            rel: "noopener noreferrer",
                            class: "card-link",
                            "Contribute \u{2192}"
                        }
                    }
                }
                div { class: "project-card",
                    div { class: "card-category", "One-Time" }
                    h3 { class: "card-title", "Buy Me a Coffee" }
                    p { class: "card-summary", "Quick one-off support through Buy Me a Coffee." }
                    div { class: "card-actions",
                        a {
                            href: BMC_URL,
                            target: "_blank",
                            rel: "noopener noreferrer",
                            class: "card-link",
                            "buymeacoffee.com/scadoshi \u{2192}"
                        }
                    }
                }
                div { class: "project-card",
                    div { class: "card-category", "Recurring" }
                    h3 { class: "card-title", "GitHub Sponsors" }
                    p { class: "card-summary", "Recurring monthly support via GitHub Sponsors." }
                    div { class: "card-actions",
                        a {
                            href: GITHUB_SPONSORS_URL,
                            target: "_blank",
                            rel: "noopener noreferrer",
                            class: "card-link",
                            "github.com/sponsors/scadoshi \u{2192}"
                        }
                    }
                }
            }
        }
    }
}
