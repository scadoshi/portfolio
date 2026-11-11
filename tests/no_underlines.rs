//! The link gate: no link on the site is underlined. A text link reads as one by the accent
//! color alone, as on zwipe.net and app.zwipe.net; the kit's sheets carry the same gate in
//! orangui.

const SHEET: &str = include_str!("../assets/main.css");

/// `sheet` with its comments blanked out, line breaks kept so line numbers still match.
fn without_comments(sheet: &str) -> String {
    let mut out = String::with_capacity(sheet.len());
    let mut rest = sheet;
    while let Some(start) = rest.find("/*") {
        let (before, after) = rest.split_at(start);
        out.push_str(before);
        let end = after
            .find("*/")
            .map_or(after.len(), |e| e.saturating_add(2));
        let (comment, tail) = after.split_at(end);
        out.extend(comment.chars().filter(|c| *c == '\n'));
        rest = tail;
    }
    out.push_str(rest);
    out
}

#[test]
fn no_rule_underlines_a_link() {
    for (number, line) in without_comments(SHEET).lines().enumerate() {
        assert!(
            !line.contains("underline"),
            "main.css:{}: {}",
            number.saturating_add(1),
            line.trim()
        );
    }
}
