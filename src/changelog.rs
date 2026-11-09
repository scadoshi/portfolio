//! Cairn's release history, rendered at `/projects/cairn/changelog`.
//!
//! Cairn makes no network requests, so nothing serves this to the app; the
//! site is the only place it shows. The working copy is cairn's
//! `context/progress/changelog.md`: every entry added there lands here in the
//! same sitting, in [`UPCOMING`] under the next version, reworded to zwipe's
//! changelog rules, which the tests below hold. At cut time, when cairn's
//! Unreleased entries move under a dated version heading, the version moves
//! to the top of [`RELEASES`] with that day as a [`Date`] and becomes the
//! "Latest". A release's date is the day its build was cut and submitted,
//! never moved to the day it went live, and matches its heading in cairn's
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
    /// The day its build was cut and submitted, the same as its heading in
    /// cairn's changelog. `None`
    /// while upcoming.
    pub date: Option<Date>,
    /// User-facing release notes, one phrase per entry.
    pub entries: &'static [&'static str],
}

/// Entries for the next version, not yet cut, newest first. Rendered at the
/// top with an "Upcoming" badge instead of "Latest".
pub(crate) const UPCOMING: &[Release] = &[Release {
    version: "1.4.1",
    date: None,
    entries: &[
        "Opening another counter from the menu shows its figures right away instead of the last counter's",
        "On a counter's screen the menu lights only that counter, not Home as well",
        "Past a counter's goal, its Goal card counts how far past (+494 past goal) and needed per day reads goal met, instead of negative figures",
        "After Save marks a field in the counter form, its message goes as soon as the field is right, instead of waiting for the next Save",
    ],
}];

/// Shipped releases, newest first by date.
pub(crate) const RELEASES: &[Release] = &[
    Release {
        version: "1.4.0",
        date: Some(Date::new(2026, 11, 8)),
        entries: &[
            "Home and a counter's screen move to the new day at the rollover hour and pick up today's figures when you come back to the app",
            "A tap after the day turns counts on the new day, so a minus no longer takes from yesterday's figure",
            "Cairn opens on Android, keeping its counters in the app's own storage",
            "Settings has a Backup row that saves every counter, its settings and every tap in one file and imports it back, asking before it replaces a counter you already have",
            "When iOS closes Cairn's page in the background to free memory, Cairn draws itself again when you come back instead of showing a frozen screen",
            "Every theme, light and dark, draws its titles, labels, buttons and errors clearly enough to read",
            "On a phone, small buttons answer a tap that lands just outside them",
            "Toasts appear at the bottom left instead of over the header",
            "A plain tap, a theme pick, the dark toggle and a Settings change no longer raise a toast",
            "On Android the keyboard no longer covers a sheet's checkmark",
            "A best's date or month stays on one line",
            "Export says 1 file when it writes one and no longer leaves a log file beside your counters",
            "Each Settings row keeps its name, value and buttons on one line on a phone",
        ],
    },
    Release {
        version: "1.3.0",
        date: Some(Date::new(2026, 11, 5)),
        entries: &[
            "The header is the shared nav bar: the title and its ? in the middle, a hamburger on the right, a back arrow on the left on every screen but Home; it slides away as you scroll down, its title and buttons fading into it, and back as you scroll up",
            "The hamburger opens a menu that lists your counters, tap one to open it, with a footer of icons: Home, Settings, an i with the version and a link to this changelog, a palette for the theme and a sun or moon for Dark/Light, with the ! at the left end",
            "Config is now Settings",
            "The ! no longer links the changelog",
            "Settings no longer shows the version",
            "The bottom footers are gone; Create floats as a plus at the bottom right of Home",
            "A counter's Edit is a floating pencil and Delete sits in a Danger zone panel",
            "The Themes sheet saves a theme as you tap it and stays open, with no checkmark",
            "The menu keeps a little room under its last counter above the footer",
            "The menu's counters scroll with no scrollbar showing",
            "Step buttons sit inside their counter's card and panel, with a pencil to edit",
            "Sheets close with an x or a drag down and save from a checkmark at their bottom right",
            "Goal animation's Test is a play button beside the checkmark",
            "The quote panel is titled Featured quote, with the countdown to the next quote beside the title and only the author and source under the quote",
            "The counter form outlines the field an error belongs to and puts the message under it",
            "Light themes keep every chip and toast readable, and the page fills to the bottom edge",
            "Hints show an icon button as the icon itself, in the button's own squircle, instead of naming it in words",
            "Home before any counter reads Create a counter with the button that starts one",
            "Settings' Theme row puts the palette on its Change button, as the Dark button carries the moon",
            "The Themes sheet groups its themes under chips",
            "The menu and the sheets drop the grid of squares behind their contents",
        ],
    },
    Release {
        version: "1.2.0",
        date: Some(Date::new(2026, 11, 1)),
        entries: &["Links open inside Cairn instead of switching to Safari"],
    },
    Release {
        version: "1.1.4",
        date: Some(Date::new(2026, 10, 29)),
        entries: &["The header's ! links to this changelog"],
    },
    Release {
        version: "1.1.3",
        date: Some(Date::new(2026, 10, 28)),
        entries: &["Settings names the theme's mode Dark or Light, capitalized"],
    },
    Release {
        version: "1.1.2",
        date: Some(Date::new(2026, 10, 28)),
        entries: &[
            "The changelog's version chips group patches under their minor version",
            "A quote that turns while the app stays open types out in full with its tags",
            "The heatmap's total and logged days sit under it as tags, and on the phone a tapped day's count and date replace them for a few seconds",
            "A counter's lifetime number sits in a box, matching the tiles under it",
            "A counter made today reads lifetime, without a since date",
        ],
    },
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
