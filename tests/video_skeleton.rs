//! A video's box is its file's shape from the first paint, a pulsing skeleton in zwipe's ghost
//! fill until its first frame can paint, and the video fades in over it.

/// The site's stylesheet.
const CSS: &str = include_str!("../assets/main.css");

/// The body of the first rule whose selector starts a line as `selector`.
fn rule<'a>(selector: &str) -> Option<&'a str> {
    let at = CSS.find(&format!("\n{selector} {{"))?;
    let body = CSS.get(at..)?;
    body.get(body.find('{')?.saturating_add(1)..body.find('}')?)
}

#[test]
fn a_video_box_takes_its_files_shape() {
    let video = rule(".gallery-video").expect("the video box");
    assert!(
        video.contains("aspect-ratio: var(--media-w) / var(--media-h)"),
        "{video}"
    );
    assert!(
        video.contains(
            "width: min(100%, calc(var(--media-max-h) * var(--media-w) / var(--media-h)))"
        ),
        "the box keeps the media's height bound: {video}"
    );
}

#[test]
fn a_pending_video_is_a_skeleton_and_fades_in_after() {
    let pending = rule(".gallery-video-pending").expect("the pending box");
    assert!(
        pending.contains("background-color: var(--border-muted)"),
        "{pending}"
    );
    assert!(
        pending.contains("animation: skeleton-pulse 2s cubic-bezier(0.4, 0, 0.6, 1) infinite"),
        "{pending}"
    );
    let hidden = rule(".gallery-video-pending > .gallery-image").expect("the hidden video");
    assert!(hidden.contains("opacity: 0"), "{hidden}");
    assert!(
        hidden.contains("animation: none"),
        "the fade starts when the box stops pending: {hidden}"
    );
    let image = rule(".gallery-image").expect("the media rule");
    assert!(image.contains("animation: gallery-fade-in"), "{image}");
}
