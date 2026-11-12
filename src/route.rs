//! The site's routes.

// Routable's derive builds each `String` field's parse error variant from an
// `Infallible`, and rustc reads that constructor as an unreachable call. The
// derive's items sit beside the enum, so only a module-level expect reaches them.
#![expect(
    unreachable_code,
    reason = "Routable's derive wraps String fields' Infallible parse errors"
)]

use dioxus::prelude::*;

use crate::pages::changelog::CairnChangelog;
use crate::pages::contribute::Contribute;
use crate::pages::detail::ProjectDetail;
use crate::pages::home::Home;
use crate::pages::moved::{OldSideQuest, OldSideQuests};
use crate::pages::not_found::NotFound;
use crate::pages::privacy::CairnPrivacy;
use crate::pages::projects::{Projects, SectionPage};
use crate::{NavbarLayout, data};

/// Every page on the site, by address. The order matters: the router tries
/// routes top to bottom, so a fixed path sits above the parameter that would
/// also match it.
#[derive(Debug, Clone, Routable, PartialEq)]
#[expect(
    missing_docs,
    reason = "each variant's #[route] path says what it is, and each field is that path's parameter"
)]
#[rustfmt::skip]
pub enum Route {
    #[layout(NavbarLayout)]
        #[route("/")]
        Home {},
        #[route("/projects/")]
        Projects {},
        // The App Store listing's privacy policy URL and the release history.
        // Two segments, so neither collides with a section or a project slug.
        #[route("/projects/cairn/privacy/")]
        CairnPrivacy {},
        #[route("/projects/cairn/changelog/")]
        CairnChangelog {},
        // Before the project route: only the four section names parse, so
        // every other slug falls through to ProjectDetail.
        #[route("/projects/:section/")]
        SectionPage { section: data::Section },
        #[route("/projects/:slug/")]
        ProjectDetail { slug: String },
        #[route("/contribute/")]
        Contribute {},
        // Side quests were folded into the sections. These addresses only
        // ever render the redirect to where each page went (data::MOVED).
        #[route("/side-quests/")]
        OldSideQuests {},
        #[route("/side-quests/:slug/")]
        OldSideQuest { slug: String },
        // GitHub Pages serves 404.html (a copy of the app shell, made in
        // deploy.yml) for unknown paths, and the router lands here.
        #[route("/:..segments")]
        NotFound { segments: Vec<String> },
}
