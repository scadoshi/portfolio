use dioxus::prelude::*;
use zwipe_components::{GalleryFooter, GalleryFrame, Panel};

use crate::data::{MediaItem, MediaKind};

#[component]
pub fn ProjectGallery(items: &'static [MediaItem]) -> Element {
    if items.is_empty() {
        return rsx! {};
    }

    let index = use_signal(|| 0usize);
    let total = items.len();
    let current = &items[index()];

    // The caption/counter footer goes through Panel's `actions` slot so it
    // pins to the bottom edge: `.panel-card` is a flex column and
    // `.panel-body` takes the slack, which only works when the footer is a
    // sibling of the body. Inside the body it sits under the image with dead
    // space below and stops bottom-aligning with its `.detail-band` neighbor.
    rsx! {
        figure { class: "project-gallery",
            Panel {
                title: "Watch it work",
                actions: rsx! {
                    GalleryFooter { index: index(), total, caption: current.caption.map(str::to_string) }
                },
                GalleryFrame { index, total, noun: "image",
                    match current.kind {
                        MediaKind::Image => rsx! {
                            img {
                                key: "{index()}",
                                class: "gallery-image",
                                src: "{current.src}",
                                alt: "{current.alt}",
                                loading: "lazy",
                            }
                        },
                        MediaKind::Video => rsx! {
                            video {
                                key: "{index()}",
                                class: "gallery-image",
                                src: "{current.src}",
                                "aria-label": "{current.alt}",
                                autoplay: true,
                                muted: true,
                                "loop": true,
                                playsinline: true,
                                controls: true,
                                preload: "metadata",
                            }
                        },
                    }
                }
            }
        }
    }
}
