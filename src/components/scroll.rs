use dioxus::prelude::*;

/// Scrolls every `.scroll-end` box to its right edge once the page is up, so
/// a time chart that scrolls on a phone opens on the newest months.
pub fn use_scroll_to_end() {
    use_effect(|| {
        spawn(async {
            let _ = document::eval(
                "for (const el of document.querySelectorAll('.scroll-end')) el.scrollLeft = el.scrollWidth;",
            )
            .await;
        });
    });
}
