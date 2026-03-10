use dioxus::prelude::*;

/// Renders text with any https:// URLs turned into clickable links.
#[component]
pub fn LinkedText(text: String) -> Element {
    let parts = split_urls(&text);
    rsx! {
        for part in parts {
            match part {
                TextPart::Plain(s) => rsx! { "{s}" },
                TextPart::Url(url) => rsx! {
                    a {
                        href: "{url}",
                        target: "_blank",
                        rel: "noopener noreferrer",
                        "{url}"
                    }
                },
            }
        }
    }
}

enum TextPart {
    Plain(String),
    Url(String),
}

fn split_urls(text: &str) -> Vec<TextPart> {
    let mut parts = Vec::new();
    let mut remaining = text;

    while let Some(start) = remaining.find("https://") {
        if start > 0 {
            parts.push(TextPart::Plain(remaining[..start].to_string()));
        }

        let url_text = &remaining[start..];
        // URL ends at whitespace, closing paren, or end of string
        let end = url_text
            .find(|c: char| c.is_whitespace() || c == ')' || c == '>' || c == ']')
            .unwrap_or(url_text.len());

        parts.push(TextPart::Url(url_text[..end].to_string()));
        remaining = &remaining[start + end..];
    }

    if !remaining.is_empty() {
        parts.push(TextPart::Plain(remaining.to_string()));
    }

    parts
}
