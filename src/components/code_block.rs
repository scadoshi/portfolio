use dioxus::prelude::*;

include!(concat!(env!("OUT_DIR"), "/highlighted.rs"));

/// The highlighted HTML build.rs made for a snippet's trimmed code.
pub(crate) fn highlighted(code: &str) -> Option<&'static str> {
    HIGHLIGHTED
        .iter()
        .find(|(source, _)| *source == code)
        .map(|(_, html)| *html)
}

#[component]
pub fn CodeBlock(title: String, code: String, description: String, lang: String) -> Element {
    let trimmed = code.trim();

    rsx! {
        div { class: "code-block",
            div { class: "code-header", "{title}" }
            pre { class: "code-content",
                if let Some(html) = highlighted(trimmed) {
                    code { class: "language-{lang}", dangerous_inner_html: html }
                } else {
                    code { class: "language-{lang}", "{trimmed}" }
                }
            }
            if !description.is_empty() {
                p { class: "code-description", "{description}" }
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::data;

    #[test]
    fn every_snippet_on_the_site_is_highlighted() {
        let mut missing = Vec::new();
        for project in data::all_projects() {
            for snippet in project.snippets {
                if highlighted(snippet.code.trim()).is_none() {
                    missing.push(format!("{} / {}", project.slug, snippet.title));
                }
            }
        }
        assert!(
            missing.is_empty(),
            "snippets build.rs did not highlight: {missing:?}"
        );
    }
}
