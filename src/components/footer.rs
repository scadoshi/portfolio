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
                " is Rust, compiled to WebAssembly with Dioxus. Its numbers come live from "
                a { href: "https://github.com/scadoshi/heron", "heron" }
                ", counted from each repository after every push, with heron's last answer built in as a fallback."
            }
        }
    }
}
