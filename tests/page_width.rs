//! The site's pages share one column, 1400px at most (Scotty's pick between the old 74rem column
//! and the window's width), running text filling its panel across it, and the nav logo takes
//! orangui's shared size.

/// The site's stylesheet.
const CSS: &str = include_str!("../assets/main.css");

/// The body of the first rule whose selector starts a line as `selector`.
fn rule<'a>(selector: &str) -> Option<&'a str> {
    let at = CSS.find(&format!("\n{selector} {{"))?;
    let body = CSS.get(at..)?;
    body.get(body.find('{')?.saturating_add(1)..body.find('}')?)
}

#[test]
fn every_page_shares_one_1400px_column() {
    let root = rule(":root").expect("the root rule");
    assert!(root.contains("--page-max: 87.5rem"), "{root}");
    for page in [".home-band", ".project-detail", ".index-page"] {
        let body = rule(page).expect("the page's rule");
        assert!(
            body.contains("max-width: var(--page-max)"),
            "{page}: {body}"
        );
    }
    let hero = rule(".hero-panel").expect("the hero rule");
    assert!(
        hero.contains("max-width: calc(var(--page-max) - 3rem)"),
        "the hero's edges align with the band's content: {hero}"
    );
}

#[test]
fn running_text_fills_its_panel() {
    assert!(
        !CSS.contains("70ch"),
        "no text is capped to a reading measure"
    );
    let copy = rule(".kit-phone-copy").unwrap_or_default();
    assert!(
        !copy.contains("max-width"),
        "the kit page's copy beside the phone: {copy}"
    );
}

#[test]
fn the_nav_logo_takes_the_kits_shared_size() {
    assert!(
        rule(".nav-logo").is_none(),
        "orangui's .nav-logo sets the mark's size and color for every site"
    );
}
