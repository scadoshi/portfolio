use dioxus::prelude::*;

/// Renders text with `[label](https://url)` markdown links and bare https:// URLs
/// turned into clickable links.
#[component]
pub fn LinkedText(text: String) -> Element {
    let parts = split_urls(&text);
    rsx! {
        for part in parts {
            match part {
                TextPart::Plain(s) => rsx! { "{s}" },
                TextPart::Url { href, label } => rsx! {
                    a {
                        href: "{href}",
                        target: "_blank",
                        rel: "noopener noreferrer",
                        "{label}"
                    }
                },
            }
        }
    }
}

enum TextPart {
    Plain(String),
    Url { href: String, label: String },
}

fn split_urls(text: &str) -> Vec<TextPart> {
    let mut parts = Vec::new();
    let mut plain_start = 0;
    let mut i = 0;

    while i < text.len() {
        let rest = &text[i..];

        // Try markdown link: [label](https://url)
        if let Some(after_bracket) = rest.strip_prefix('[') {
            if let Some(close_bracket) = after_bracket.find(']') {
                let after_label = &after_bracket[close_bracket + 1..];
                if after_label.starts_with("(https://") {
                    if let Some(close_paren) = after_label.find(')') {
                        let label = &after_bracket[..close_bracket];
                        let href = &after_label[1..close_paren];
                        if i > plain_start {
                            parts.push(TextPart::Plain(text[plain_start..i].to_string()));
                        }
                        parts.push(TextPart::Url {
                            href: href.to_string(),
                            label: label.to_string(),
                        });
                        i += 1 + close_bracket + 1 + close_paren + 1;
                        plain_start = i;
                        continue;
                    }
                }
            }
        }

        // Try bare URL: https://...
        if rest.starts_with("https://") {
            // URL ends at whitespace or any trailing punctuation that's clearly not part of a URL
            let end = rest
                .find(|c: char| {
                    c.is_whitespace() || matches!(c, ')' | '>' | ']' | ';' | ',' | '"' | '\'')
                })
                .unwrap_or(rest.len());
            // Strip a trailing '.' so "site.com." doesn't include the sentence period
            let end = if end > 0 && rest.as_bytes()[end - 1] == b'.' {
                end - 1
            } else {
                end
            };
            if i > plain_start {
                parts.push(TextPart::Plain(text[plain_start..i].to_string()));
            }
            let url = rest[..end].to_string();
            parts.push(TextPart::Url {
                href: url.clone(),
                label: url,
            });
            i += end;
            plain_start = i;
            continue;
        }

        i += text[i..].chars().next().map(|c| c.len_utf8()).unwrap_or(1);
    }

    if plain_start < text.len() {
        parts.push(TextPart::Plain(text[plain_start..].to_string()));
    }

    parts
}
