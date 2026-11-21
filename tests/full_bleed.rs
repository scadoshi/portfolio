//! A full-bleed box in a panel runs out through the panel's side padding to its border and no
//! further: its negative side margin names the panel padding token, so the two move together.

const SHEET: &str = include_str!("../assets/main.css");

/// The body of the rule for `selector` in the site's sheet.
fn rule(selector: &str) -> &'static str {
    let Some(at) = SHEET.find(&format!("\n{selector} {{")) else {
        return "";
    };
    SHEET
        .get(at..)
        .and_then(|rest| rest.split_once('}'))
        .map_or("", |(body, _)| body)
}

#[test]
fn the_orangutan_band_bleeds_to_the_panel_s_border() {
    let frame = rule(".orangutan-frame");
    assert!(
        frame.contains("margin: 0 calc(-1 * var(--panel-pad));"),
        "out by the panel's padding, to its border: {frame:?}"
    );
}
