use dioxus::prelude::*;
use orangui::Panel;

use crate::components::page_meta::PageMeta;

/// Where the App Store listing points. Linked from the cairn project page.
pub(crate) const CAIRN_PRIVACY_PATH: &str = "/projects/cairn/privacy";

#[component]
pub fn CairnPrivacy() -> Element {
    rsx! {
        PageMeta {
            title: "Cairn privacy policy",
            description: "Cairn collects nothing. Everything you enter stays on your phone; there is no account, no server and no analytics.",
            path: CAIRN_PRIVACY_PATH,
        }
        div { class: "index-page content-enter",
            Panel {
                eyebrow: "Cairn",
                title: "Privacy policy",
                // Page hero, so the title is this page's h1 (see detail.rs).
                title_h1: true,
                p { class: "card-summary",
                    "Cairn collects nothing. Everything you enter stays on your phone, in a database inside the app's own folder."
                }
                p { class: "card-summary",
                    "There is no account, no server, no analytics, no crash reporting, and no third-party code that phones home. The app makes no network requests at all."
                }
                p { class: "card-summary",
                    "The one way data leaves the device is Export, which writes a CSV of two columns, day and count, to a place you choose, when you tap it."
                }
                p { class: "card-summary",
                    "Deleting the app deletes the data. Export first if you want to keep it."
                }
                p { class: "card-summary",
                    "Questions go to "
                    a { href: "mailto:scottyfermo@hotmail.com", "scottyfermo@hotmail.com" }
                    ". Effective 18 October 2026. If any of this changes, the change will be here and in the app's release notes."
                }
            }
        }
    }
}
