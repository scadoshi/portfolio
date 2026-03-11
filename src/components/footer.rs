use dioxus::prelude::*;

#[component]
pub fn Footer() -> Element {
    rsx! {
        footer { class: "footer",
            div { class: "footer-container",
                span { "\u{00a9} 2026 Scotty Fermo" }
                div { class: "footer-links",
                    a {
                        href: "https://github.com/scadoshi",
                        target: "_blank",
                        rel: "noopener noreferrer",
                        "GitHub"
                    }
                    a {
                        href: "https://www.linkedin.com/in/scotty-fermo-41a35b141/",
                        target: "_blank",
                        rel: "noopener noreferrer",
                        "LinkedIn"
                    }
                    a {
                        href: "mailto:scottyfermo@hotmail.com",
                        "Email"
                    }
                }
            }
        }
    }
}
