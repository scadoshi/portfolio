//! The hover gate: a hover look applies only where the device hovers. Every `:hover` rule in the
//! site's sheet sits inside `@media (hover: hover)`, so a tap on a touch screen does not leave
//! what it touched lit until the next tap lands elsewhere; the kit's sheets carry the same gate
//! in orangui.

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

/// Every block's prelude in `sheet` with its line and the preludes of the blocks around it.
fn blocks(sheet: &str) -> Vec<(usize, String, Vec<String>)> {
    let mut found = Vec::new();
    let mut open: Vec<String> = Vec::new();
    let mut prelude = String::new();
    let mut line = 1usize;
    for ch in without_comments(sheet).chars() {
        match ch {
            '{' => {
                let selector = prelude.split_whitespace().collect::<Vec<_>>().join(" ");
                found.push((line, selector.clone(), open.clone()));
                open.push(selector);
                prelude.clear();
            }
            '}' => {
                open.pop();
                prelude.clear();
            }
            ';' => prelude.clear(),
            '\n' => {
                line = line.saturating_add(1);
                prelude.push(' ');
            }
            other => prelude.push(other),
        }
    }
    found
}

#[test]
fn every_hover_look_waits_for_a_device_that_hovers() {
    for (line, selector, around) in blocks(SHEET) {
        if !selector.contains(":hover") || selector.starts_with('@') {
            continue;
        }
        let gated = around
            .iter()
            .any(|p| p.starts_with("@media") && p.contains("(hover: hover)"));
        assert!(
            gated,
            "main.css:{line}: `{selector}` is outside @media (hover: hover)"
        );
    }
}
