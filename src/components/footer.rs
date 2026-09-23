use dioxus::prelude::*;

#[component]
pub fn Footer() -> Element {
    rsx! {
        footer { class: "footer",
            p { class: "footer-copy",
                "\u{00a9} 2026 Scotty Fermo | "
                a {
                    href: "https://github.com/scadoshi",
                    "GitHub" span { class: "ext", "\u{2197}" }
                }
                " | "
                a {
                    href: "https://www.linkedin.com/in/scotty-fermo-41a35b141/",
                    "LinkedIn" span { class: "ext", "\u{2197}" }
                }
                " | "
                a {
                    href: "mailto:scottyfermo@hotmail.com",
                    "Email" span { class: "ext", "\u{2197}" }
                }
            }
            p { class: "footer-built-text",
                a {
                    class: "footer-built-repo",
                    href: "https://github.com/scadoshi/portfolio",
                    "This site"
                }
                " is unofficial "
                span { class: "footer-built-strong", "JavaScript-Free Content" }
                // The line count is real (assets/reveal.js). If that file
                // grows, this number is a lie; the last one stood false for
                // fifteen days.
                ", hand-written in Rust and compiled to WebAssembly via Dioxus. Not approved or endorsed by the ECMAScript committee. Fifty-four lines of JavaScript fade the panels in, and one library visits on a day pass to color the code snippets. Both are escorted at all times."
            }
        }
    }
}
