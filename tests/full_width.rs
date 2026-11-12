//! The portfolio runs the window's width: the hero, the bands and the index
//! and detail pages share one width with no fixed cap, running text keeps a
//! readable measure, and the home band takes columns of panels from 1920px.

const STYLE: &str = include_str!("../assets/main.css");

/// Every top-level rule whose selector line is exactly `selector`, joined.
fn rule(selector: &str) -> String {
    let lines: Vec<&str> = STYLE.lines().collect();
    let open = format!("{selector} {{");
    let found: Vec<String> = lines
        .iter()
        .enumerate()
        .filter(|(_, line)| line.trim_end() == open)
        .map(|(at, _)| {
            lines
                .iter()
                .skip(at)
                .take_while(|line| *line != &"}")
                .copied()
                .collect::<Vec<_>>()
                .join("\n")
        })
        .collect();
    assert!(!found.is_empty(), "no rule {selector}");
    found.join("\n")
}

#[test]
fn every_page_shares_the_wide_page_s_width() {
    assert!(rule(":root").contains("--pf-page-max: calc(7 * 26rem"));
    for selector in [
        ".hero-panel",
        ".home-band",
        ".project-detail",
        ".index-page",
    ] {
        let body = rule(selector);
        assert!(
            body.contains("max-width: var(--pf-page-max)"),
            "{selector}: {body}"
        );
        assert!(!body.contains("max-width: 74rem"), "{selector}: {body}");
    }
}

#[test]
fn running_text_keeps_a_readable_measure() {
    assert!(STYLE.contains(
        ".theme-wrapper :is(.home-band, .index-page, .project-detail) .panel-card p {\n    max-width: 70ch;"
    ));
}

#[test]
fn the_home_band_takes_columns_from_1920() {
    let at = STYLE.find("@media (min-width: 120rem) {").unwrap();
    let block = STYLE.get(at..).unwrap();
    assert!(block.contains("columns: 26rem 7;"), "{block}");
}
