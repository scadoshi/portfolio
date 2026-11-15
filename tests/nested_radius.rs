//! Nested corners: a rounded box inside another takes the outer radius minus the border and the
//! inset between them, so the gap between the two curves stays even around the corner.

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
fn the_phone_glass_follows_the_bezel() {
    let phone = rule(".phone");
    assert!(phone.contains("--phone-radius: 2.6rem;"), "{phone:?}");
    assert!(phone.contains("--phone-inset: 0.55rem;"), "{phone:?}");
    assert!(
        phone.contains("border-radius: var(--phone-radius);"),
        "{phone:?}"
    );
    assert!(phone.contains("padding: var(--phone-inset);"), "{phone:?}");
    let glass = rule(".phone-viewport");
    assert!(
        glass.contains(
            "border-radius: max(0.15rem, calc(var(--phone-radius) - 1px - var(--phone-inset)));"
        ),
        "the glass is the bezel's radius less its border and padding: {glass:?}"
    );
}
