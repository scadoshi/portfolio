use dioxus::prelude::*;
use zwipe_components::{PageMeta as SharedPageMeta, SiteMeta};

/// This site's constants for the shared head-meta component. No OG image,
/// which also selects the plain `summary` Twitter card.
const SITE: SiteMeta = SiteMeta {
    base_url: "https://scottyfermo.com",
    site_name: "Scotty Fermo",
    og_image_path: None,
};

/// Thin wrapper over the shared [`SharedPageMeta`]: bakes in the site config
/// so pages keep calling `PageMeta { title, description, path }` unchanged.
/// The home page passes the bare site name and renders unsuffixed, same as
/// before (the shared component's bare-brand rule).
#[component]
pub fn PageMeta(title: String, description: String, path: String) -> Element {
    rsx! {
        SharedPageMeta { site: SITE, title, description, path }
    }
}
