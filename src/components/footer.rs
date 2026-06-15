use dioxus::prelude::*;

#[component]
pub fn Footer() -> Element {
    rsx! {
        footer { class: "footer",
            p { class: "footer-copy",
                "\u{00a9} 2026 Scotty Fermo | "
                a {
                    href: "https://github.com/scadoshi",
                    target: "_blank",
                    rel: "noopener noreferrer",
                    "GitHub"
                }
                " | "
                a {
                    href: "https://www.linkedin.com/in/scotty-fermo-41a35b141/",
                    target: "_blank",
                    rel: "noopener noreferrer",
                    "LinkedIn"
                }
                " | "
                a {
                    href: "mailto:scottyfermo@hotmail.com",
                    "Email"
                }
            }
            p { class: "footer-built-text",
                a {
                    class: "footer-built-repo",
                    href: "https://github.com/scadoshi/portfolio",
                    target: "_blank",
                    rel: "noopener noreferrer",
                    "This site"
                }
                " is unofficial "
                span { class: "footer-built-strong", "JavaScript-Free Content" }
                ", hand-written in Rust and compiled to WebAssembly via Dioxus. Not approved or endorsed by the ECMAScript committee. No JavaScript was shipped, bundled, or harmed in the making of this page."
            }
        }
    }
}
