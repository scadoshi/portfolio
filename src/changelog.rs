//! Cairn's release history, rendered at `/projects/cairn/changelog`.
//!
//! Cairn makes no network requests, so nothing serves this to the app; the
//! site is the only place it shows. The working copy is cairn's
//! `context/progress/changelog.md`: every entry added there lands here in the
//! same sitting, in [`UPCOMING`] under the next version, reworded to zwipe's
//! changelog rules, which the tests below hold. At cut time the version moves
//! to [`RELEASES`] with its date.

/// One version and its notes. Dates are the App Store's "Ready for
/// Distribution" date; newest first.
pub struct Release {
    /// Semantic version string, e.g. `"1.1.0"`.
    pub version: &'static str,
    /// Human-readable release date, e.g. `"Oct 20, 2026"`. Empty while
    /// upcoming.
    pub date: &'static str,
    /// User-facing release notes, one phrase per entry.
    pub entries: &'static [&'static str],
}

/// Versions built but not yet on the App Store, newest first. Rendered at
/// the top with an "Upcoming" badge instead of "Latest".
pub const UPCOMING: &[Release] = &[
    Release {
        version: "1.1.1",
        date: "",
        entries: &[
            "Flexoki theme replaces Visual Studio, whose users move to VS Code",
            "Synthwave '84's colors match the original theme",
        ],
    },
    Release {
        version: "1.1.0",
        date: "",
        entries: &[
            "Four Monokai Pro themes: Pro, Machine, Octagon and Spectrum, each in dark and light",
            "Editor defaults as themes: Darcula, Xcode, Visual Studio, Eclipse and Neovim, each in dark and light",
            "Ten more themes: Bluloco, GitHub High Contrast, Kanagawa Dragon, Material, Melange, Modus, Nightfox, Oxocarbon, Poimandres and Terafox, each in dark and light, bringing the set to 50",
            "The theme and App icon pickers group themes under headings by look, with high contrast and color blind last",
            "Ristretto is now called Monokai Pro Ristretto",
            "The App icon picker offers the new themes too",
            "The backdrop is a field of shaded squares instead of grid lines, on screens and sheets",
            "The app opens on a short splash: the backdrop rains in and the C resolves over it before the first screen, whose cards, figures and quote arrive as it leaves",
            "The app icon can match your theme, switching with it in dark and light",
            "The app no longer flashes unstyled for a moment before the splash",
            "The logo builds left to right, its shape arriving as noise and then decoding into the mark, on the splash and the home screen",
            "The splash's logo leaves the way it arrived, turning back to noise and clearing left to right before the home screen shows",
            "The splash spells out the full Cairn wordmark",
        ],
    },
];

/// Shipped releases, newest first.
pub const RELEASES: &[Release] = &[Release {
    version: "1.0.0",
    date: "Oct 20, 2026",
    entries: &[
        "Counters that keep a lifetime total, with a breakdown by year",
        "Goals per day, week or year, with pace: what today still owes, ahead or behind, and the projected year end",
        "A goal tag on each card counting down to the day's share, green once met",
        "A step per counter, from 1 to 100, and an optional big step beside it",
        "Trend charts: this week, 60 days with a 7-day average, weekly totals, monthly averages, weekday pattern and hour of day",
        "Every day of the last year as a grid, each day shaded by how it ranks",
        "A numbers card: streaks, consistency, the last 7 days against the 7 before, and best week and month",
        "Rest days that don't count against consistency or break a streak",
        "Six goal animations that play when a counter meets its daily goal, set app-wide or per counter",
        "A quote that turns over on the hour",
        "31 themes, each in dark and light, and an App icon picker to match",
        "CSV export, one file per counter",
        "No account and no server: everything stays on the phone",
    ],
}];

#[cfg(test)]
mod tests {
    use super::*;

    fn every_entry() -> impl Iterator<Item = (&'static str, &'static str)> {
        UPCOMING.iter().chain(RELEASES).flat_map(|release| {
            release
                .entries
                .iter()
                .map(move |entry| (release.version, *entry))
        })
    }

    /// An entry is a phrase, not a sentence: no period, no exclamation, no
    /// trailing ellipsis.
    #[test]
    fn no_entry_ends_with_a_period() {
        let offenders: Vec<_> = every_entry()
            .filter(|(_, entry)| entry.ends_with(['.', '!', '\u{2026}']))
            .collect();
        assert!(offenders.is_empty(), "{offenders:#?}");
    }

    /// An entry says what changed and stops.
    #[test]
    fn no_entry_gives_an_example() {
        let offenders: Vec<_> = every_entry()
            .filter(|(_, entry)| {
                entry.contains(" like ")
                    || entry.contains("e.g.")
                    || entry.contains("for example")
                    || entry.contains("such as")
            })
            .collect();
        assert!(offenders.is_empty(), "{offenders:#?}");
    }

    #[test]
    fn no_entry_is_empty_or_uses_an_em_dash() {
        let offenders: Vec<_> = every_entry()
            .filter(|(_, entry)| entry.trim().is_empty() || entry.contains('\u{2014}'))
            .collect();
        assert!(offenders.is_empty(), "{offenders:#?}");
    }

    /// Only shipped releases carry a date, and every one of them does.
    #[test]
    fn only_releases_are_dated() {
        assert!(UPCOMING.iter().all(|r| r.date.is_empty()));
        assert!(RELEASES.iter().all(|r| !r.date.is_empty()));
    }
}
