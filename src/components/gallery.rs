use dioxus::prelude::*;
use orangui::{GalleryFooter, GalleryFrame, Panel};

use crate::data::{MediaItem, MediaKind};

#[component]
pub fn ProjectGallery(items: &'static [MediaItem]) -> Element {
    if items.is_empty() {
        return rsx! {};
    }

    let index = use_signal(|| 0usize);
    let total = items.len();
    let Some(current) = items.get(index()) else {
        return rsx! {};
    };

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
                        MediaKind::Video { width, height } => rsx! {
                            GalleryVideo {
                                key: "{index()}",
                                item: current.clone(),
                                width,
                                height,
                            }
                        },
                    }
                }
            }
        }
    }
}

/// The class of a video's box: a pulsing skeleton until the video settles,
/// then the video fades in over it.
fn video_box_class(settled: bool) -> &'static str {
    if settled {
        "gallery-video"
    } else {
        "gallery-video gallery-video-pending"
    }
}

/// A video in a box its own `width` by `height` shape, so nothing moves when
/// it arrives. The box pulses as a skeleton and the video stays hidden until
/// it settles: its first frame can paint, or it fails to load, so a broken
/// file shows the player's own error state instead of pulsing forever.
#[component]
fn GalleryVideo(item: MediaItem, width: u32, height: u32) -> Element {
    let mut settled = use_signal(|| false);
    rsx! {
        div {
            class: video_box_class(settled()),
            style: "--media-w: {width}; --media-h: {height};",
            video {
                class: "gallery-image",
                src: "{item.src}",
                "aria-label": "{item.alt}",
                width: "{width}",
                height: "{height}",
                autoplay: true,
                muted: true,
                "loop": true,
                playsinline: true,
                controls: true,
                preload: "metadata",
                onloadeddata: move |_| settled.set(true),
                onerror: move |_| settled.set(true),
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn a_video_box_is_a_skeleton_until_the_video_settles() {
        assert_eq!(
            video_box_class(false),
            "gallery-video gallery-video-pending"
        );
        assert_eq!(video_box_class(true), "gallery-video");
    }
}
