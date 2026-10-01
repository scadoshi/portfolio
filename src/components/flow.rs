use dioxus::prelude::*;

/// A labeled box in the diagram.
fn node(x: f64, y: f64, w: f64, title: &str, sub: &str, class: &str) -> Element {
    rsx! {
        g { class: "flow-node {class}",
            rect { x: "{x}", y: "{y}", width: "{w}", height: "52", rx: "8" }
            text { class: "flow-title", x: "{x + w / 2.0}", y: "{y + 22.0}", text_anchor: "middle", "{title}" }
            text { class: "flow-sub", x: "{x + w / 2.0}", y: "{y + 40.0}", text_anchor: "middle", "{sub}" }
        }
    }
}

/// An arrow from one box edge to another, with its timing beside the line: above
/// a level arrow, and pushed off to the side of a diagonal one.
fn arrow(x1: f64, y1: f64, x2: f64, y2: f64, label: &str) -> Element {
    let diagonal = (y2 - y1).abs() > 1.0;
    // A label on a rising arrow sits left of the midpoint, on a falling one right.
    let shift = if diagonal {
        -30.0 * (y2 - y1).signum()
    } else {
        0.0
    };
    let mx = f64::midpoint(x1, x2) + shift;
    let my = f64::midpoint(y1, y2) - if diagonal { 14.0 } else { 6.0 };
    rsx! {
        g { class: "flow-arrow",
            line { x1: "{x1}", y1: "{y1}", x2: "{x2}", y2: "{y2}", marker_end: "url(#flow-head)" }
            text { x: "{mx}", y: "{my}", text_anchor: "middle", "{label}" }
        }
    }
}

/// How a number reaches this site: GitHub and a tarball into heron, heron into
/// steller and back, and heron out to the site's build and to the browser. Inline
/// so the boxes take the theme's colors.
#[component]
pub fn Flow() -> Element {
    rsx! {
        svg {
            class: "flow",
            view_box: "0 0 720 236",
            role: "img",
            "aria-label": "GitHub and a tarball feed heron, heron keeps snapshots in steller, and the site's build and the browser read heron",
            defs {
                marker {
                    id: "flow-head",
                    view_box: "0 0 10 10",
                    ref_x: "9",
                    ref_y: "5",
                    marker_width: "7",
                    marker_height: "7",
                    orient: "auto-start-reverse",
                    path { d: "M 0 0 L 10 5 L 0 10 z" }
                }
            }
            {node(10.0, 20.0, 150.0, "GitHub API", "commits, stars, pushes", "flow-source")}
            {node(10.0, 164.0, 150.0, "tarball of main", "the source itself", "flow-source")}
            {node(285.0, 20.0, 150.0, "heron", "refresh + sweep", "flow-heron")}
            {node(285.0, 164.0, 150.0, "steller", "snapshots, 7 days", "flow-cache")}
            {node(550.0, 20.0, 160.0, "portfolio deploy", "bakes stats.json", "flow-site")}
            {node(550.0, 164.0, 160.0, "browser", "asks after load", "flow-site")}
            {arrow(160.0, 40.0, 285.0, 40.0, "every 15 min")}
            {arrow(160.0, 176.0, 285.0, 62.0, "after a push")}
            {arrow(435.0, 40.0, 550.0, 40.0, "each build")}
            {arrow(435.0, 62.0, 550.0, 176.0, "5 min at the edge")}
            g { class: "flow-arrow",
                line {
                    x1: "360", y1: "76", x2: "360", y2: "160",
                    marker_start: "url(#flow-head)",
                    marker_end: "url(#flow-head)",
                }
            }
        }
    }
}
