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

    while let Some(rest) = text.get(i..).filter(|rest| !rest.is_empty()) {
        let found = markdown_link(rest).or_else(|| {
            bare_url(rest).map(|url| {
                let part = TextPart::Url {
                    href: url.to_string(),
                    label: url.to_string(),
                };
                (part, url.len())
            })
        });
        if let Some((part, used)) = found {
            push_plain(&mut parts, text, plain_start, i);
            parts.push(part);
            i = i.saturating_add(used);
            plain_start = i;
            continue;
        }
        i = i.saturating_add(rest.chars().next().map_or(1, char::len_utf8));
    }

    if let Some(tail) = text.get(plain_start..).filter(|tail| !tail.is_empty()) {
        parts.push(TextPart::Plain(tail.to_string()));
    }

    parts
}

/// A `[label](https://...)` link at the start of `rest`, with how many bytes it
/// spans.
fn markdown_link(rest: &str) -> Option<(TextPart, usize)> {
    let (label, after_label) = rest.strip_prefix('[')?.split_once(']')?;
    let body = after_label.strip_prefix('(')?;
    if !body.starts_with("https://") {
        return None;
    }
    let (href, tail) = body.split_once(')')?;
    let part = TextPart::Url {
        href: href.to_string(),
        label: label.to_string(),
    };
    Some((part, rest.len().saturating_sub(tail.len())))
}

/// A bare `https://` URL at the start of `rest`, up to whitespace or punctuation
/// that can't be part of it, without a sentence's trailing period.
fn bare_url(rest: &str) -> Option<&str> {
    if !rest.starts_with("https://") {
        return None;
    }
    let end = rest
        .find(|c: char| c.is_whitespace() || matches!(c, ')' | '>' | ']' | ';' | ',' | '"' | '\''))
        .unwrap_or(rest.len());
    let url = rest.get(..end)?;
    Some(url.strip_suffix('.').unwrap_or(url))
}

/// Pushes the plain text between `start` and `end`, if there is any.
fn push_plain(parts: &mut Vec<TextPart>, text: &str, start: usize, end: usize) {
    if let Some(plain) = text.get(start..end).filter(|plain| !plain.is_empty()) {
        parts.push(TextPart::Plain(plain.to_string()));
    }
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
