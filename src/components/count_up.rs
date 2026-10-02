use dioxus::prelude::*;

use crate::{components::chart::Replay, stats::with_separators};

/// How long a number takes to reach its value, and the tick between frames.
const DURATION_MS: f64 = 1000.0;
const TICK_MS: u32 = 16;

/// A number that counts up from zero when the page opens and again on every
/// replay, easing out so the last digits settle. Prerendered at its final
/// value, so the page reads right before any script runs. A value that
/// changes mid-count, the live snapshot arriving, becomes the new target.
#[component]
pub fn CountUp(value: u64) -> Element {
    let replay = use_context::<Replay>().0;
    let mut target = use_signal(|| value);
    let mut shown = use_signal(|| value);
    use_effect(use_reactive!(|value| target.set(value)));
    use_effect(move || {
        let run = replay();
        spawn(async move {
            let frames = (DURATION_MS / f64::from(TICK_MS)).ceil();
            let mut frame = 0.0;
            while frame < frames {
                gloo_timers::future::TimeoutFuture::new(TICK_MS).await;
                // A later replay owns the number now.
                if replay.peek().ne(&run) {
                    return;
                }
                frame += 1.0;
                let t = (frame / frames).min(1.0);
                let eased = 1.0 - (1.0 - t).powi(3);
                shown.set(scaled(*target.peek(), eased));
            }
            shown.set(*target.peek());
        });
    });
    rsx! { "{with_separators(shown())}" }
}

/// `value` at `fraction` of the way up, never past it. Counts are small, so
/// the float conversion is exact.
#[allow(
    clippy::cast_precision_loss,
    clippy::cast_possible_truncation,
    clippy::cast_sign_loss
)]
fn scaled(value: u64, fraction: f64) -> u64 {
    ((value as f64) * fraction.clamp(0.0, 1.0)).round() as u64
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn the_number_rises_to_its_value_and_no_further() {
        assert_eq!(scaled(3_657, 0.0), 0);
        assert_eq!(scaled(3_657, 0.5), 1_829);
        assert_eq!(scaled(3_657, 1.0), 3_657);
        assert_eq!(scaled(3_657, 1.5), 3_657);
    }
}
