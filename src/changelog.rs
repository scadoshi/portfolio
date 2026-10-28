//! Cairn's release history, rendered at `/projects/cairn/changelog`.
//!
//! Cairn makes no network requests, so nothing serves this to the app; the
//! site is the only place it shows. The working copy is cairn's
//! `context/progress/changelog.md`: every entry added there lands here in the
//! same sitting, in [`UPCOMING`] under the next version, reworded to zwipe's
//! changelog rules, which the tests below hold. At cut time, when cairn's
//! Unreleased entries move under a dated version heading, the version moves
//! to the top of [`RELEASES`] with that day as a [`Date`] and becomes the
//! "Latest". A release's date is the day App Store Connect shows it "Ready
//! for Distribution", when it went live, and matches its heading in cairn's
//! changelog. The page files each release under the year and month of that
//! date (see `components::changelog`).

use std::fmt;

/// A calendar day. Fields run year, month, day, so the derived ordering is
/// chronological. Built with [`Date::new`], which rejects an impossible day
/// at compile time when used in a const.
#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub(crate) struct Date {
    year: u16,
    month: u8,
    day: u8,
}

impl Date {
    /// The day `year`-`month`-`day`. Panics, which in a const is a compile
    /// error, when the month is not 1 to 12 or the day is not in that month.
    pub(crate) const fn new(year: u16, month: u8, day: u8) -> Self {
        assert!(month >= 1 && month <= 12, "month out of range");
        assert!(
            day >= 1 && day <= days_in_month(year, month),
            "day out of range"
        );
        Self { year, month, day }
    }

    /// The first of this date's month, the key the page files releases under.
    pub(crate) const fn month_start(self) -> Self {
        Self { day: 1, ..self }
    }

    /// The year, e.g. `2026`.
    pub(crate) const fn year(self) -> u16 {
        self.year
    }

    /// The month alone, e.g. `"Oct"`. A month chip's label.
    pub(crate) const fn month_name(self) -> &'static str {
        month_abbr(self.month)
    }
}

/// Formats as `"Oct 21, 2026"`, the release date shown on each card.
impl fmt::Display for Date {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{} {}, {}", month_abbr(self.month), self.day, self.year)
    }
}

const fn month_abbr(month: u8) -> &'static str {
    match month {
        1 => "Jan",
        2 => "Feb",
        3 => "Mar",
        4 => "Apr",
        5 => "May",
        6 => "Jun",
        7 => "Jul",
        8 => "Aug",
        9 => "Sep",
        10 => "Oct",
        11 => "Nov",
        _ => "Dec",
    }
}

const fn days_in_month(year: u16, month: u8) -> u8 {
    match month {
        2 if year.is_multiple_of(4) && (!year.is_multiple_of(100) || year.is_multiple_of(400)) => {
            29
        }
        2 => 28,
        4 | 6 | 9 | 11 => 30,
        _ => 31,
    }
}

/// One version and its notes.
pub(crate) struct Release {
    /// Semantic version string, e.g. `"1.1.0"`.
    pub version: &'static str,
    /// The day App Store Connect showed the version "Ready for
    /// Distribution", the same as its heading in cairn's changelog. `None`
    /// while upcoming.
    pub date: Option<Date>,
    /// User-facing release notes, one phrase per entry.
    pub entries: &'static [&'static str],
}

/// Entries for the next version, not yet cut, newest first. Rendered at the
/// top with an "Upcoming" badge instead of "Latest".
pub(crate) const UPCOMING: &[Release] = &[Release {
    version: "1.1.2",
    date: None,
    entries: &[
        "The changelog's version chips group patches under their minor version",
        "A quote that turns while the app stays open types out in full with its tags",
        "The heatmap's total and logged days sit under it as tags, and on the phone a tapped day's count and date replace them for a few seconds",
        "A counter's lifetime number sits in a box like the tiles under it",
    ],
}];

/// Shipped releases, newest first by date.
pub(crate) const RELEASES: &[Release] = &[
    Release {
        version: "1.1.1",
        date: Some(Date::new(2026, 10, 21)),
        entries: &[
            "Flexoki theme replaces Visual Studio, whose users move to VS Code",
            "Synthwave '84's colors match the original theme",
        ],
    },
    Release {
        version: "1.1.0",
        date: Some(Date::new(2026, 10, 21)),
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
    Release {
        version: "1.0.0",
        date: Some(Date::new(2026, 10, 20)),
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
    },
];

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
        assert!(UPCOMING.iter().all(|r| r.date.is_none()));
        assert!(RELEASES.iter().all(|r| r.date.is_some()));
    }

    /// The page reads top to bottom as newest to oldest, and the first
    /// release is the "Latest", so the order is held here rather than by eye.
    #[test]
    fn releases_are_newest_first_by_date() {
        let dates: Vec<_> = RELEASES.iter().filter_map(|r| r.date).collect();
        assert!(
            dates.windows(2).all(|pair| pair[0] >= pair[1]),
            "{dates:#?}"
        );
    }

    #[test]
    fn dates_read_the_way_the_page_shows_them() {
        let date = Date::new(2026, 10, 21);
        assert_eq!(date.to_string(), "Oct 21, 2026");
        assert_eq!(date.month_name(), "Oct");
        assert_eq!(date.month_start(), Date::new(2026, 10, 1));
        assert_eq!(Date::new(2024, 2, 29).to_string(), "Feb 29, 2024");
    }
}
