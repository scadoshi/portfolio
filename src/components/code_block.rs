use dioxus::prelude::*;

#[component]
pub fn CodeBlock(title: String, code: String, description: String) -> Element {
    let trimmed = code.trim().to_string();
    rsx! {
        div { class: "code-block",
            div { class: "code-header", "{title}" }
            pre { class: "code-content",
                code { "{trimmed}" }
            }
            if !description.is_empty() {
                p { class: "code-description", "{description}" }
            }
        }
    }
}
