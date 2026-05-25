use dioxus::prelude::*;

use crate::data::{MediaItem, MediaKind};

#[component]
pub fn ProjectGallery(items: &'static [MediaItem]) -> Element {
    if items.is_empty() {
        return rsx! {};
    }

    let mut index = use_signal(|| 0usize);
    let total = items.len();
    let current = &items[index()];

    rsx! {
        figure { class: "project-gallery",
            div { class: "gallery-frame",
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
            div { class: "gallery-meta",
                if let Some(caption) = current.caption {
                    figcaption { class: "gallery-caption", "{caption}" }
                }
                if total > 1 {
                    span { class: "gallery-counter", "{index() + 1} / {total}" }
                }
            }
        }
    }
}
