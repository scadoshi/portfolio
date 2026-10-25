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
                        "{label}"
                    }
                },
            }
        }
    }
}

#[derive(Debug, PartialEq)]
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

        if let Some(after_bracket) = rest.strip_prefix('[')
            && let Some(close_bracket) = after_bracket.find(']')
        {
            let after_label = &after_bracket[close_bracket + 1..];
            if after_label.starts_with("(https://")
                && let Some(close_paren) = after_label.find(')')
            {
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

        if rest.starts_with("https://") {
            // Stop at whitespace or punctuation that can't be part of the URL.
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

        i += text[i..].chars().next().map_or(1, char::len_utf8);
    }

    if plain_start < text.len() {
        parts.push(TextPart::Plain(text[plain_start..].to_string()));
    }

    parts
}

#[cfg(test)]
mod tests {
    use super::*;

    fn plain(s: &str) -> TextPart {
        TextPart::Plain(s.to_string())
    }

    fn url(href: &str, label: &str) -> TextPart {
        TextPart::Url {
            href: href.to_string(),
            label: label.to_string(),
        }
    }

    #[test]
    fn plain_text_is_one_part() {
        assert_eq!(split_urls("no links here"), vec![plain("no links here")]);
        assert_eq!(split_urls(""), vec![]);
    }

    #[test]
    fn a_markdown_link_becomes_its_label() {
        assert_eq!(
            split_urls("see [the repo](https://github.com/x) now"),
            vec![
                plain("see "),
                url("https://github.com/x", "the repo"),
                plain(" now")
            ]
        );
    }

    #[test]
    fn a_bare_url_stops_at_whitespace_and_punctuation() {
        assert_eq!(
            split_urls("at https://a.dev, then"),
            vec![
                plain("at "),
                url("https://a.dev", "https://a.dev"),
                plain(", then")
            ]
        );
        for stop in [")", ">", "]", ";", "\"", "'"] {
            assert_eq!(
                split_urls(&format!("https://a.dev{stop}x")),
                vec![
                    url("https://a.dev", "https://a.dev"),
                    plain(&format!("{stop}x"))
                ],
                "stop {stop:?}"
            );
        }
    }

    #[test]
    fn a_trailing_period_stays_out_of_the_url() {
        assert_eq!(
            split_urls("go to https://a.dev."),
            vec![
                plain("go to "),
                url("https://a.dev", "https://a.dev"),
                plain(".")
            ]
        );
        assert_eq!(
            split_urls("https://."),
            vec![url("https://", "https://"), plain(".")]
        );
    }

    #[test]
    fn brackets_without_an_https_link_stay_plain() {
        assert_eq!(split_urls("[a](http://x)"), vec![plain("[a](http://x)")]);
        assert_eq!(split_urls("[unclosed"), vec![plain("[unclosed")]);
        assert_eq!(
            split_urls("[a](https://x"),
            vec![plain("[a]("), url("https://x", "https://x")]
        );
    }

    #[test]
    fn links_sit_among_non_ascii_text() {
        assert_eq!(
            split_urls("café [ünï](https://ü.dev) – https://b.dev ✓"),
            vec![
                plain("café "),
                url("https://ü.dev", "ünï"),
                plain(" – "),
                url("https://b.dev", "https://b.dev"),
                plain(" ✓"),
            ]
        );
    }

    #[test]
    fn two_links_in_a_row_have_no_empty_part_between() {
        assert_eq!(
            split_urls("[a](https://a)[b](https://b)"),
            vec![url("https://a", "a"), url("https://b", "b")]
        );
    }
}
