//! Sweeps a newly picked theme across the page from left to right.
//!
//! The picker writes a pending theme rather than the real one. A view
//! transition snapshots the page, the wrapper's theme class is swapped inside
//! it, and the real theme is committed only once the swap is in, so the
//! re-render lands on a page that already shows it. The stylesheet's
//! `theme-wipe` keyframes reveal the new snapshot over the old. Browsers
//! without view transitions, and readers who ask for reduced motion, get the
//! instant swap.

use dioxus::prelude::*;
use zwipe_components::ThemeConfig;

/// Receives the new theme class, swaps the wrapper onto it inside a view
/// transition, and answers once the swap is in. Answers straight away when it
/// cannot wipe, leaving the swap to the re-render.
const WIPE_JS: &str = r#"
const next = await dioxus.recv();
const wrapper = document.querySelector(".theme-wrapper");
const still = matchMedia("(prefers-reduced-motion: reduce)").matches;
if (wrapper && document.startViewTransition && !still) {
    const swap = () => {
        Array.from(wrapper.classList).forEach((c) => {
            if (c.startsWith("theme-") && c !== "theme-wrapper") wrapper.classList.remove(c);
        });
        wrapper.classList.add(next);
    };
    try {
        await document.startViewTransition(swap).updateCallbackDone;
    } catch (e) {}
}
dioxus.send(true);
"#;

/// The signal to hand the theme picker in place of `theme`.
///
/// A pick wipes in and then becomes `theme`. A change to `theme` from
/// elsewhere, such as the stored theme adopted after mount, moves the pick
/// with it and does not wipe.
pub fn use_theme_wipe(mut theme: Signal<ThemeConfig>) -> Signal<ThemeConfig> {
    let mut picked = use_signal(|| theme.peek().clone());

    use_effect(move || {
        let next = picked.read().clone();
        if *theme.peek() == next {
            return;
        }
        spawn(async move {
            let mut js = document::eval(WIPE_JS);
            let _ = js.send(next.css_class());
            let _ = js.recv::<bool>().await;
            // A later pick supersedes this one and commits itself.
            if *picked.peek() == next {
                theme.set(next);
            }
        });
    });

    use_effect(move || {
        let current = theme.read().clone();
        if *picked.peek() != current {
            picked.set(current);
        }
    });

    picked
}
