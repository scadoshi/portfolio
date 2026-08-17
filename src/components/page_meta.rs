use dioxus::prelude::*;
use zwipe_components::{PageMeta as SharedPageMeta, SiteMeta};

/// This site's constants for the shared head-meta component. The OG image
/// (which also selects the `summary_large_image` Twitter card) lives in
/// public/ so dx serves it verbatim at this literal URL; regenerate it from
/// context/marketing/og_default.html (instructions in that file).
const SITE: SiteMeta = SiteMeta {
    base_url: "https://scottyfermo.com",
    site_name: "Scotty Fermo",
    og_image_path: Some("/assets/og-default.png"),
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
