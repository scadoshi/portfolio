use dioxus::prelude::*;
use zwipe_components::{DiagramArrow, DiagramDefs, DiagramNode, DiagramTone};

/// How a number reaches this site: GitHub and a tarball into heron, heron into
/// steller and back, and heron out to the site's build and to the browser. Inline
/// so the boxes take the theme's colors.
#[component]
pub fn Flow() -> Element {
    rsx! {
        div { class: "diagram-scroll",
        svg {
            class: "diagram",
            view_box: "0 0 720 236",
            role: "img",
            "aria-label": "GitHub and a tarball feed heron, heron keeps snapshots in steller, and the site's build and the browser read heron",
            DiagramDefs {}
            DiagramNode { x: 10.0, y: 20.0, title: "GitHub API", sub: "commits, stars, pushes", tone: DiagramTone::Muted }
            DiagramNode { x: 10.0, y: 164.0, title: "tarball of main", sub: "the source itself", tone: DiagramTone::Muted }
            DiagramNode { x: 285.0, y: 20.0, title: "heron", sub: "refresh + sweep", tone: DiagramTone::Primary }
            DiagramNode { x: 285.0, y: 164.0, title: "steller", sub: "snapshots, 7 days", tone: DiagramTone::Tertiary }
            DiagramNode { x: 550.0, y: 20.0, w: 160.0, title: "portfolio deploy", sub: "bakes stats.json", tone: DiagramTone::Secondary }
            DiagramNode { x: 550.0, y: 164.0, w: 160.0, title: "browser", sub: "asks after load", tone: DiagramTone::Secondary }
            DiagramArrow { x1: 160.0, y1: 40.0, x2: 285.0, y2: 40.0, label: "every 15 min" }
            DiagramArrow { x1: 160.0, y1: 176.0, x2: 285.0, y2: 62.0, label: "after a push" }
            DiagramArrow { x1: 435.0, y1: 40.0, x2: 550.0, y2: 40.0, label: "each build" }
            DiagramArrow { x1: 435.0, y1: 62.0, x2: 550.0, y2: 176.0, label: "5 min at the edge" }
            DiagramArrow { x1: 360.0, y1: 76.0, x2: 360.0, y2: 160.0, both: true }
        }
        }
    }
}
