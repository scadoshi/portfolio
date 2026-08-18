use dioxus::prelude::*;
use zwipe_components::Panel;

use crate::data::{MediaItem, MediaKind};

#[component]
pub fn ProjectGallery(items: &'static [MediaItem]) -> Element {
    if items.is_empty() {
        return rsx! {};
    }

    let mut index = use_signal(|| 0usize);
    let total = items.len();
    let current = &items[index()];

    // Same Panel grammar as zite's demo gallery: eyebrow + title head, the
    // media body, then the caption/counter footer. The footer goes through
    // Panel's `actions` slot rather than being written into the body, so it
    // pins to the panel's bottom edge: `.panel-card` is a flex column and
    // `.panel-body` takes the slack, which only pushes the footer down when
    // the footer is a sibling of the body. Written into the body instead, it
    // sits right under the image with the dead space below it, and stops
    // bottom-aligning with the panel beside it in a `.detail-band`.
    rsx! {
        figure { class: "project-gallery",
            Panel {
                eyebrow: "Demo",
                title: "Watch it work",
                actions: rsx! {
                    div { class: "gallery-footer",
                        if let Some(caption) = current.caption {
                            figcaption { class: "gallery-caption", "{caption}" }
                        }
                        if total > 1 {
                            span { class: "gallery-counter", "{index() + 1} / {total}" }
                        }
                    }
                },
                div { class: "gallery-body",
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
                    if total > 1 {
                        button {
                            class: "gallery-nav gallery-prev",
                            aria_label: "Previous image",
                            onclick: move |_| {
                                let i = index();
                                index.set(if i == 0 { total - 1 } else { i - 1 });
                            },
                            "\u{2190}"
                        }
                        button {
                            class: "gallery-nav gallery-next",
                            aria_label: "Next image",
                            onclick: move |_| {
                                let i = index();
                                index.set((i + 1) % total);
                            },
                            "\u{2192}"
                        }
                    }
                }
            }
        }
    }
}
