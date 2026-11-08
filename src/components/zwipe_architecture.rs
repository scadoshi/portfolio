//! Zwipe's architecture as a two-slide carousel in a panel: how the eight
//! crates and orangui import each other, then how the running pieces talk.
//! A copy of zwipe's `components::architecture`, which zite shows; change both.
//!
//! Inline SVG through orangui's diagram pieces, so the boxes take the theme's
//! colors; the frame and footer are the demo gallery's ([`fn@GalleryFrame`],
//! [`fn@GalleryFooter`]), restyled for short wide diagrams by `.diagram-carousel`
//! in orangui's site sheet.

use dioxus::prelude::*;
use orangui::{
    DiagramArrow, DiagramDefs, DiagramNode, DiagramTone, GalleryFooter, GalleryFrame, Panel,
};

/// The caption under each slide, in slide order.
const CAPTIONS: [&str; 2] = [
    "Eight crates in one workspace over orangui, the UI kit I built for all my apps: the app, the web app and the site share one typed client, one core, the screen logic and the Magic UI, and the server shares the core",
    "Three clients, one server: the app, the web app and the site call the same API, and the card catalog syncs from Scryfall nightly",
];

/// The first slide's description for screen readers.
const CRATES_LABEL: &str = "zwiper, zwiper-web and zite import the shared crates, zerver imports zwipe-core, zwipe-client imports zwipe-core, zwipe-ui imports zwipe-core, components imports zwipe-ui and zwipe-core, and zwipe-ui and components import orangui, the UI kit in its own repo";

/// The second slide's description for screen readers.
const RUNTIME_LABEL: &str = "zwiper, zwiper-web and zite call zerver over HTTPS; zerver reads and writes PostgreSQL and pulls the card catalog from Scryfall nightly";

/// The carousel in a panel. `title` names the panel; the eyebrow is
/// "Architecture".
#[component]
pub fn ZwipeArchitecture(
    #[props(default = "How it fits together".to_string())] title: String,
) -> Element {
    let index = use_signal(|| 0usize);
    let total = CAPTIONS.len();
    rsx! {
        Panel {
            eyebrow: "Architecture",
            title,
            actions: rsx! {
                GalleryFooter { index: index(), total, caption: CAPTIONS.get(index()).map(|c| (*c).to_string()) }
            },
            GalleryFrame { index, total, noun: "diagram", class: "diagram-carousel".to_string(),
                div { key: "{index()}", class: "diagram-slide diagram-scroll",
                    if index() == 0 {
                        Crates {}
                    } else {
                        Infrastructure {}
                    }
                }
            }
        }
    }
}

/// Which crate imports which: the three clients over the shared crates, the server
/// over the core alone, the screen logic and the Magic UI over orangui, the
/// kit in its own repo.
#[component]
fn Crates() -> Element {
    rsx! {
        svg {
            class: "diagram",
            view_box: "0 0 720 330",
            role: "img",
            "aria-label": CRATES_LABEL,
            DiagramDefs {}
            DiagramNode { x: 25.0, y: 16.0, title: "zwiper", sub: "iOS and Android app", tone: DiagramTone::Primary }
            DiagramNode { x: 200.0, y: 16.0, title: "zerver", sub: "Axum, the REST API", tone: DiagramTone::Tertiary }
            DiagramNode { x: 375.0, y: 16.0, title: "zwiper-web", sub: "app.zwipe.net", tone: DiagramTone::Primary }
            DiagramNode { x: 550.0, y: 16.0, title: "zite", sub: "zwipe.net", tone: DiagramTone::Primary }
            g { class: "diagram-group",
                rect { x: "10", y: "136", width: "700", height: "92", rx: "10" }
                text { x: "24", y: "224", "shared crates, imported by all three clients" }
            }
            DiagramNode { x: 20.0, y: 150.0, w: 160.0, title: "zwipe-client", sub: "typed API calls", tone: DiagramTone::Secondary }
            DiagramNode { x: 195.0, y: 150.0, w: 160.0, title: "zwipe-core", sub: "models, filters, traits", tone: DiagramTone::Success }
            DiagramNode { x: 370.0, y: 150.0, w: 160.0, title: "zwipe-ui", sub: "any client's screens", tone: DiagramTone::Secondary }
            DiagramNode { x: 545.0, y: 150.0, w: 160.0, title: "components", sub: "the Magic UI", tone: DiagramTone::Secondary }
            DiagramNode { x: 457.0, y: 262.0, w: 160.0, title: "orangui", sub: "UI kit, its own repo", tone: DiagramTone::Secondary }
            DiagramArrow { x1: 100.0, y1: 68.0, x2: 100.0, y2: 136.0, label: "imports" }
            DiagramArrow { x1: 275.0, y1: 68.0, x2: 275.0, y2: 150.0, label: "imports" }
            DiagramArrow { x1: 450.0, y1: 68.0, x2: 450.0, y2: 136.0, label: "imports" }
            DiagramArrow { x1: 625.0, y1: 68.0, x2: 625.0, y2: 136.0, label: "imports" }
            DiagramArrow { x1: 180.0, y1: 176.0, x2: 195.0, y2: 176.0 }
            DiagramArrow { x1: 370.0, y1: 176.0, x2: 355.0, y2: 176.0 }
            DiagramArrow { x1: 545.0, y1: 176.0, x2: 530.0, y2: 176.0 }
            DiagramArrow { x1: 450.0, y1: 202.0, x2: 500.0, y2: 262.0, label: "imports" }
            DiagramArrow { x1: 625.0, y1: 202.0, x2: 575.0, y2: 262.0, label: "imports" }
        }
    }
}

/// The running system: the three clients call the server, which keeps PostgreSQL
/// and pulls the catalog from Scryfall.
#[component]
fn Infrastructure() -> Element {
    rsx! {
        svg {
            class: "diagram",
            view_box: "0 0 720 196",
            role: "img",
            "aria-label": RUNTIME_LABEL,
            DiagramDefs {}
            DiagramNode { x: 10.0, y: 16.0, title: "Scryfall", sub: "the card catalog", tone: DiagramTone::Muted }
            DiagramNode { x: 285.0, y: 16.0, title: "zerver", sub: "Axum, the REST API", tone: DiagramTone::Tertiary }
            DiagramNode { x: 560.0, y: 16.0, title: "PostgreSQL", sub: "SQLx", tone: DiagramTone::Muted }
            DiagramNode { x: 10.0, y: 130.0, title: "zwiper", sub: "iOS and Android", tone: DiagramTone::Primary }
            DiagramNode { x: 285.0, y: 130.0, title: "zwiper-web", sub: "app.zwipe.net", tone: DiagramTone::Primary }
            DiagramNode { x: 560.0, y: 130.0, title: "zite", sub: "zwipe.net", tone: DiagramTone::Primary }
            DiagramArrow { x1: 160.0, y1: 42.0, x2: 285.0, y2: 42.0, label: "nightly" }
            DiagramArrow { x1: 435.0, y1: 42.0, x2: 560.0, y2: 42.0, label: "reads and writes" }
            DiagramArrow { x1: 110.0, y1: 130.0, x2: 310.0, y2: 68.0, label: "HTTPS" }
            DiagramArrow { x1: 360.0, y1: 130.0, x2: 360.0, y2: 68.0, label: "HTTPS" }
            DiagramArrow { x1: 610.0, y1: 130.0, x2: 410.0, y2: 68.0, label: "HTTPS" }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    /// The workspace's crates, as zwipe's root `Cargo.toml` lists them;
    /// every one is a box on the first slide.
    const WORKSPACE: [&str; 8] = [
        "zwiper",
        "zwiper-web",
        "zite",
        "zerver",
        "zwipe-client",
        "zwipe-core",
        "zwipe-ui",
        "components",
    ];

    /// Counts in words, as a caption opens with one.
    const WORDS: [&str; 11] = [
        "Zero", "One", "Two", "Three", "Four", "Five", "Six", "Seven", "Eight", "Nine", "Ten",
    ];

    #[test]
    fn the_crates_slide_names_every_crate_and_counts_them() {
        let count = WORDS[WORKSPACE.len()];
        assert!(
            CAPTIONS[0].starts_with(&format!("{count} crates")),
            "{}",
            CAPTIONS[0]
        );
        for name in WORKSPACE {
            assert!(
                CRATES_LABEL.contains(name),
                "{name} missing from the crates slide"
            );
        }
    }

    #[test]
    fn the_runtime_slide_names_every_client() {
        for client in ["zwiper,", "zwiper-web", "zite"] {
            assert!(RUNTIME_LABEL.contains(client), "{client}");
        }
        assert!(CAPTIONS[1].starts_with("Three clients"), "{}", CAPTIONS[1]);
    }
}
