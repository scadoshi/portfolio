//! Cairn's changelog page: a drill-down of chips over the release cards.
//!
//! Row one is Latest and one chip per year. Picking a year opens a row of its
//! months, and picking a month a row of the minor versions filed in it, e.g.
//! `1.1`. Each pick shows the cards under it: a whole year, a month, or every
//! patch of one minor in that month. Latest, the default, shows the upcoming
//! version, if any, and the newest release. Every card's id is its
//! [`anchor`], so `/projects/cairn/changelog#v1-0-0` opens on that release's
//! year, month and minor and scrolls to its card.

use dioxus::prelude::*;
use orangui::Chip;

use crate::changelog::{Date, RELEASES, Release, UPCOMING};

/// The chip picked last, the deepest level of the drill-down. The levels
/// above it follow from it (see [`Selection::path`]).
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(crate) enum Selection {
    /// The upcoming version and the newest release.
    Latest,
    /// Every release in a year.
    Year(u16),
    /// Every release in a month, given as its first day.
    Month(Date),
    /// Every release of a minor version, e.g. `"1.1"`, filed in a month.
    /// The month is part of the pick because a minor can span months.
    Minor {
        /// The month, given as its first day.
        month: Date,
        /// The minor version, major and minor only.
        minor: &'static str,
    },
}

impl Selection {
    /// The chips this selection lights: its own and every level above it.
    pub(crate) fn path(self) -> Path {
        match self {
            Self::Latest => Path::default(),
            Self::Year(year) => Path {
                year: Some(year),
                ..Path::default()
            },
            Self::Month(month) => Path {
                year: Some(month.year()),
                month: Some(month),
                minor: None,
            },
            Self::Minor { month, minor } => Path {
                year: Some(month.year()),
                month: Some(month),
                minor: Some(minor),
            },
        }
    }

    /// One level up: minor to its month, month to its year, year to Latest.
    pub(crate) fn parent(self) -> Self {
        match self {
            Self::Latest | Self::Year(_) => Self::Latest,
            Self::Month(month) => Self::Year(month.year()),
            Self::Minor { month, .. } => Self::Month(month),
        }
    }

    /// The selection after a tap on `chip`. A chip above the current pick
    /// selects its level and clears the rows below it; tapping the chip
    /// already picked steps back up one level.
    pub(crate) fn tap(self, chip: Self) -> Self {
        if chip == self { self.parent() } else { chip }
    }
}

/// The chips a selection lights, one per level. Empty under Latest.
#[derive(Debug, Default, PartialEq, Eq)]
pub(crate) struct Path {
    /// The year chip lit, if any.
    pub year: Option<u16>,
    /// The month chip lit, if any.
    pub month: Option<Date>,
    /// The minor version chip lit, if any.
    pub minor: Option<&'static str>,
}

/// A product's changelog, upcoming versions and shipped releases, both
/// newest first, and the drill-down over them.
#[derive(Clone, Copy)]
pub(crate) struct Log {
    upcoming: &'static [Release],
    releases: &'static [Release],
}

impl Log {
    /// Cairn's changelog, from `crate::changelog`.
    pub(crate) const CAIRN: Self = Self::new(UPCOMING, RELEASES);

    /// A changelog over these upcoming versions and shipped releases.
    pub(crate) const fn new(upcoming: &'static [Release], releases: &'static [Release]) -> Self {
        Self { upcoming, releases }
    }

    /// Upcoming versions, then shipped releases: the page's order.
    fn all(self) -> impl Iterator<Item = &'static Release> {
        self.upcoming.iter().chain(self.releases)
    }

    /// Whether `version` is an upcoming one, which gets the Upcoming badge.
    pub(crate) fn is_upcoming(self, version: &str) -> bool {
        self.upcoming
            .iter()
            .any(|release| release.version == version)
    }

    /// Whether `version` is the newest shipped release, which gets the
    /// Latest badge.
    pub(crate) fn is_latest(self, version: &str) -> bool {
        self.releases
            .first()
            .is_some_and(|release| release.version == version)
    }

    /// The month a release is filed under: the month of its date, or for an
    /// upcoming version, which has none, the newest month any release
    /// shipped in.
    fn month_of(self, release: &Release) -> Option<Date> {
        release.date.map(Date::month_start).or_else(|| {
            self.releases
                .iter()
                .filter_map(|r| r.date)
                .max()
                .map(Date::month_start)
        })
    }

    /// The months with a release, newest first.
    fn months_all(self) -> Vec<Date> {
        let mut months: Vec<Date> = self.all().filter_map(|r| self.month_of(r)).collect();
        months.sort_unstable_by(|a, b| b.cmp(a));
        months.dedup();
        months
    }

    /// The years with a release, newest first. Row one, after Latest.
    pub(crate) fn years(self) -> Vec<u16> {
        let mut years: Vec<u16> = self.months_all().into_iter().map(Date::year).collect();
        years.dedup();
        years
    }

    /// The months of `year` with a release, newest first. Row two.
    pub(crate) fn months(self, year: u16) -> Vec<Date> {
        self.months_all()
            .into_iter()
            .filter(|month| month.year() == year)
            .collect()
    }

    /// The minor versions of the releases filed under `month`, newest first.
    /// Row three.
    pub(crate) fn minors(self, month: Date) -> Vec<&'static str> {
        let mut minors = Vec::new();
        for release in self.all() {
            let minor = minor_of(release.version);
            if self.month_of(release) == Some(month) && !minors.contains(&minor) {
                minors.push(minor);
            }
        }
        minors
    }

    /// The releases `selection` shows, in page order.
    pub(crate) fn shown(self, selection: Selection) -> Vec<&'static Release> {
        self.all()
            .filter(|release| match selection {
                Selection::Latest => {
                    self.is_upcoming(release.version) || self.is_latest(release.version)
                }
                Selection::Year(year) => self.month_of(release).map(Date::year) == Some(year),
                Selection::Month(month) => self.month_of(release) == Some(month),
                Selection::Minor { month, minor } => {
                    self.month_of(release) == Some(month) && minor_of(release.version) == minor
                }
            })
            .collect()
    }

    /// The selection a link to `fragment` opens on, with or without its
    /// leading `#`: the month and minor of the release whose [`anchor`] it
    /// is. `None` when no release has that anchor, so the page stays on
    /// Latest. An upcoming version with no shipped release to file it under
    /// opens on Latest, which shows it.
    pub(crate) fn for_anchor(self, fragment: &str) -> Option<Selection> {
        let fragment = fragment.strip_prefix('#').unwrap_or(fragment);
        let release = self
            .all()
            .find(|release| anchor(release.version) == fragment)?;
        Some(
            self.month_of(release)
                .map_or(Selection::Latest, |month| Selection::Minor {
                    month,
                    minor: minor_of(release.version),
                }),
        )
    }
}

/// The major and minor of `version`, e.g. `"1.1"` for 1.1.2. A version with
/// fewer than three parts is its own minor.
fn minor_of(version: &'static str) -> &'static str {
    version
        .match_indices('.')
        .nth(1)
        .map_or(version, |(end, _)| &version[..end])
}

/// A release card's id and the fragment that links to it: `v` and the
/// version with its dots as dashes, e.g. `"v1-1-0"` for 1.1.0.
pub(crate) fn anchor(version: &str) -> String {
    format!("v{}", version.replace('.', "-"))
}

/// Sets the sticky nav's height as the cards' scroll margin, so a jump to a
/// card stops below the nav, then scrolls to the card whose id `dioxus.recv`
/// gives, if any, once the selection has rendered it.
const SCROLL_JS: &str = r"
const nav = document.querySelector('.nav-wrapper');
const offset = (nav ? nav.getBoundingClientRect().height : 0) + 16;
document.documentElement.style.setProperty('--changelog-scroll-margin', offset + 'px');
const id = await dioxus.recv();
if (id) {
    requestAnimationFrame(() => requestAnimationFrame(() => {
        document.getElementById(id)?.scrollIntoView({ block: 'start' });
    }));
}
";

/// Cairn's release history: the chip drill-down over newest-first release
/// cards, each with its date and notes. The cards are a port of zwipe's
/// `components::Changelog`, so the two read the same.
#[component]
pub fn Changelog() -> Element {
    let log = Log::CAIRN;

    // Prerendered on Latest, so the first client render matches; the
    // fragment is only visible to the browser and is applied after mount.
    let mut selected = use_signal(|| Selection::Latest);
    use_effect(move || {
        spawn(async move {
            let mut hash = document::eval("dioxus.send(window.location.hash);");
            let fragment = hash.recv::<String>().await.unwrap_or_default();
            let target = log.for_anchor(&fragment);
            if let Some(selection) = target {
                selected.set(selection);
            }
            let scroll = document::eval(SCROLL_JS);
            let id = target.map(|_| fragment.trim_start_matches('#').to_string());
            let _ = scroll.send(id.unwrap_or_default());
        });
    });

    let selection = selected();
    let path = selection.path();
    let mut tap = move |chip: Selection| selected.set(selected().tap(chip));
    // In each card's key so a selection change remounts the visible cards,
    // replaying their ease-in.
    let selection_key = format!("{selection:?}");

    rsx! {
        div { class: "changelog-chips",
            div { class: "changelog-filter",
                Chip {
                    selected: selection == Selection::Latest,
                    onclick: move |_| tap(Selection::Latest),
                    "Latest"
                }
                for year in log.years() {
                    Chip {
                        key: "{year}",
                        selected: path.year == Some(year),
                        onclick: move |_| tap(Selection::Year(year)),
                        "{year}"
                    }
                }
            }
            if let Some(year) = path.year {
                div { class: "changelog-filter",
                    for month in log.months(year) {
                        Chip {
                            key: "{month.month_name()}",
                            selected: path.month == Some(month),
                            onclick: move |_| tap(Selection::Month(month)),
                            "{month.month_name()}"
                        }
                    }
                }
            }
            if let Some(month) = path.month {
                div { class: "changelog-filter",
                    for minor in log.minors(month) {
                        Chip {
                            key: "{minor}",
                            selected: path.minor == Some(minor),
                            onclick: move |_| tap(Selection::Minor { month, minor }),
                            "{minor}"
                        }
                    }
                }
            }
        }
        div { class: "changelog-list",
            for release in log.shown(selection) {
                div {
                    key: "{selection_key}-{release.version}",
                    id: anchor(release.version),
                    class: "changelog-card",
                    div { class: "changelog-version-row",
                        h2 { class: "changelog-version",
                            a {
                                href: "#{anchor(release.version)}",
                                title: "Link to {release.version}",
                                "{release.version}"
                            }
                        }
                        if let Some(date) = release.date {
                            span { class: "changelog-date", "{date}" }
                        }
                        if log.is_upcoming(release.version) {
                            span { class: "status-tag status-doing", "Upcoming" }
                        } else if log.is_latest(release.version) {
                            span { class: "status-tag status-done", "Latest" }
                        }
                    }
                    ul { class: "changelog-bullets",
                        for entry in release.entries {
                            li { "{entry}" }
                        }
                    }
                }
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const OCT: Date = Date::new(2026, 10, 1);
    const NOV: Date = Date::new(2026, 11, 1);
    const DEC: Date = Date::new(2026, 12, 1);
    const JAN: Date = Date::new(2027, 1, 1);

    fn versions(releases: &[&Release]) -> Vec<&'static str> {
        releases.iter().map(|r| r.version).collect()
    }

    const fn minor(month: Date, minor: &'static str) -> Selection {
        Selection::Minor { month, minor }
    }

    /// Releases across a new year, with an upcoming version: 2.1.0 is
    /// upcoming, 2.0.0 shipped in January 2027, the rest in late 2026, with
    /// 1.3 patched once in December and 1.1 spanning November and December.
    const TWO_YEARS: Log = Log::new(
        &[Release {
            version: "2.1.0",
            date: None,
            entries: &["Something next"],
        }],
        &[
            Release {
                version: "2.0.0",
                date: Some(Date::new(2027, 1, 5)),
                entries: &["Something new"],
            },
            Release {
                version: "1.3.1",
                date: Some(Date::new(2026, 12, 30)),
                entries: &["Something fixed"],
            },
            Release {
                version: "1.3.0",
                date: Some(Date::new(2026, 12, 28)),
                entries: &["Something old"],
            },
            Release {
                version: "1.2.0",
                date: Some(Date::new(2026, 12, 2)),
                entries: &["Something older"],
            },
            Release {
                version: "1.1.1",
                date: Some(Date::new(2026, 12, 1)),
                entries: &["Something patched"],
            },
            Release {
                version: "1.1.0",
                date: Some(Date::new(2026, 11, 20)),
                entries: &["Something oldest"],
            },
        ],
    );

    #[test]
    fn latest_shows_upcoming_and_the_newest_release() {
        let shown = versions(&Log::CAIRN.shown(Selection::Latest));
        let mut expected: Vec<_> = UPCOMING.iter().map(|r| r.version).collect();
        expected.extend(RELEASES.first().map(|r| r.version));
        assert_eq!(shown, expected);
        assert_eq!(
            versions(&TWO_YEARS.shown(Selection::Latest)),
            ["2.1.0", "2.0.0"]
        );
        assert_eq!(Selection::Latest.path(), Path::default());
    }

    /// The year row shows even when every release is in one year.
    #[test]
    fn one_year_still_gets_a_year_chip() {
        assert_eq!(Log::CAIRN.years(), [2026]);
        assert_eq!(Log::CAIRN.months(2026), [OCT]);
        assert_eq!(Log::CAIRN.minors(OCT), ["1.1", "1.0"]);
        assert_eq!(
            versions(&Log::CAIRN.shown(Selection::Year(2026))),
            ["1.1.2", "1.1.1", "1.1.0", "1.0.0"]
        );
    }

    #[test]
    fn minors_are_major_and_minor_only() {
        assert_eq!(minor_of("1.1.2"), "1.1");
        assert_eq!(minor_of("10.0.12"), "10.0");
        assert_eq!(minor_of("2.0"), "2.0");
        assert_eq!(minor_of("3"), "3");
    }

    #[test]
    fn each_level_narrows_the_cards() {
        let log = TWO_YEARS;
        assert_eq!(log.years(), [2027, 2026]);
        assert_eq!(log.months(2026), [DEC, NOV]);
        assert_eq!(log.months(2027), [JAN]);
        // The upcoming version files under the newest month.
        assert_eq!(log.minors(JAN), ["2.1", "2.0"]);
        assert_eq!(log.minors(DEC), ["1.3", "1.2", "1.1"]);
        assert_eq!(log.minors(NOV), ["1.1"]);

        assert_eq!(
            versions(&log.shown(Selection::Year(2026))),
            ["1.3.1", "1.3.0", "1.2.0", "1.1.1", "1.1.0"]
        );
        assert_eq!(
            versions(&log.shown(Selection::Month(DEC))),
            ["1.3.1", "1.3.0", "1.2.0", "1.1.1"]
        );
        assert_eq!(
            Selection::Month(JAN).path(),
            Path {
                year: Some(2027),
                month: Some(JAN),
                minor: None,
            }
        );
    }

    #[test]
    fn a_minor_shows_its_patches_in_that_month() {
        let log = TWO_YEARS;
        assert_eq!(versions(&log.shown(minor(DEC, "1.3"))), ["1.3.1", "1.3.0"]);
        assert_eq!(versions(&log.shown(minor(DEC, "1.2"))), ["1.2.0"]);
        // A minor that spans months shows only the month picked.
        assert_eq!(versions(&log.shown(minor(DEC, "1.1"))), ["1.1.1"]);
        assert_eq!(versions(&log.shown(minor(NOV, "1.1"))), ["1.1.0"]);
        assert_eq!(
            minor(DEC, "1.3").path(),
            Path {
                year: Some(2026),
                month: Some(DEC),
                minor: Some("1.3"),
            }
        );
    }

    #[test]
    fn a_higher_chip_clears_the_levels_below() {
        let picked = minor(DEC, "1.3");
        assert_eq!(picked.tap(Selection::Month(DEC)), Selection::Month(DEC));
        assert_eq!(picked.tap(Selection::Year(2026)), Selection::Year(2026));
        assert_eq!(picked.tap(Selection::Year(2027)), Selection::Year(2027));
        assert_eq!(picked.tap(Selection::Latest), Selection::Latest);
        assert_eq!(picked.tap(minor(DEC, "1.2")), minor(DEC, "1.2"));
    }

    #[test]
    fn tapping_the_picked_chip_steps_up_one_level() {
        let picked = minor(DEC, "1.3");
        assert_eq!(picked.tap(picked), Selection::Month(DEC));
        assert_eq!(
            Selection::Month(DEC).tap(Selection::Month(DEC)),
            Selection::Year(2026)
        );
        assert_eq!(
            Selection::Year(2026).tap(Selection::Year(2026)),
            Selection::Latest
        );
        assert_eq!(Selection::Latest.tap(Selection::Latest), Selection::Latest);
    }

    #[test]
    fn anchors_are_the_version_with_dashes() {
        assert_eq!(anchor("1.1.0"), "v1-1-0");
        assert_eq!(anchor("10.0.12"), "v10-0-12");
    }

    #[test]
    fn an_anchor_picks_its_year_month_and_minor() {
        let picked = Log::CAIRN.for_anchor("#v1-0-0");
        assert_eq!(picked, Some(minor(OCT, "1.0")));
        assert_eq!(
            minor(OCT, "1.0").path(),
            Path {
                year: Some(2026),
                month: Some(OCT),
                minor: Some("1.0"),
            }
        );
        assert_eq!(Log::CAIRN.for_anchor("v1-1-1"), Some(minor(OCT, "1.1")));
        assert_eq!(TWO_YEARS.for_anchor("#v2-1-0"), Some(minor(JAN, "2.1")));
        // The anchored card is among those its selection shows.
        for release in TWO_YEARS.all() {
            let picked = TWO_YEARS.for_anchor(&anchor(release.version));
            let shown = picked.map(|selection| versions(&TWO_YEARS.shown(selection)));
            assert!(
                shown.is_some_and(|shown| shown.contains(&release.version)),
                "{}",
                release.version
            );
        }
    }

    #[test]
    fn an_unknown_anchor_stays_on_latest() {
        for fragment in ["", "#", "#v9-9-9", "#1.0.0", "#top"] {
            assert_eq!(Log::CAIRN.for_anchor(fragment), None, "{fragment}");
        }
    }

    /// An upcoming version with no shipped release opens on Latest.
    #[test]
    fn an_upcoming_anchor_without_releases_opens_on_latest() {
        const FIRST: Log = Log::new(
            &[Release {
                version: "0.1.0",
                date: None,
                entries: &["Something first"],
            }],
            &[],
        );
        assert_eq!(FIRST.for_anchor("#v0-1-0"), Some(Selection::Latest));
        assert_eq!(versions(&FIRST.shown(Selection::Latest)), ["0.1.0"]);
    }
}
