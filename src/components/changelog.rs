//! Cairn's changelog page: a drill-down of chips over the release cards.
//!
//! Row one is Latest and one chip per year. Picking a year opens a row of its
//! months, and picking a month a row of its versions. Each pick shows the
//! cards under it: a whole year, a month, or one version. Latest, the
//! default, shows the upcoming version, if any, and the newest release.
//! Every card's id is its [`anchor`], so `/projects/cairn/changelog#v1-0-0`
//! opens on that release with its year, month and version picked.

use dioxus::prelude::*;
use orangui::Chip;

use crate::changelog::{Date, RELEASES, Release, UPCOMING};

/// The chip picked last, the deepest level of the drill-down. The levels
/// above it follow from it (see [`Log::path`]).
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Selection {
    /// The upcoming version and the newest release.
    Latest,
    /// Every release in a year.
    Year(u16),
    /// Every release in a month, given as its first day.
    Month(Date),
    /// One release.
    Version(&'static str),
}

/// The chips a selection lights, one per level. Empty under Latest.
#[derive(Debug, Default, PartialEq, Eq)]
pub struct Path {
    /// The year chip lit, if any.
    pub year: Option<u16>,
    /// The month chip lit, if any.
    pub month: Option<Date>,
    /// The version chip lit, if any.
    pub version: Option<&'static str>,
}

/// A product's changelog, upcoming versions and shipped releases, both
/// newest first, and the drill-down over them.
#[derive(Clone, Copy)]
pub struct Log {
    upcoming: &'static [Release],
    releases: &'static [Release],
}

impl Log {
    /// Cairn's changelog, from `crate::changelog`.
    pub const CAIRN: Self = Self::new(UPCOMING, RELEASES);

    /// A changelog over these upcoming versions and shipped releases.
    pub const fn new(upcoming: &'static [Release], releases: &'static [Release]) -> Self {
        Self { upcoming, releases }
    }

    /// Upcoming versions, then shipped releases: the page's order.
    fn all(self) -> impl Iterator<Item = &'static Release> {
        self.upcoming.iter().chain(self.releases)
    }

    fn find(self, version: &str) -> Option<&'static Release> {
        self.all().find(|release| release.version == version)
    }

    /// Whether `version` is an upcoming one, which gets the Upcoming badge.
    pub fn is_upcoming(self, version: &str) -> bool {
        self.upcoming
            .iter()
            .any(|release| release.version == version)
    }

    /// Whether `version` is the newest shipped release, which gets the
    /// Latest badge.
    pub fn is_latest(self, version: &str) -> bool {
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
    pub fn years(self) -> Vec<u16> {
        let mut years: Vec<u16> = self.months_all().into_iter().map(Date::year).collect();
        years.dedup();
        years
    }

    /// The months of `year` with a release, newest first. Row two.
    pub fn months(self, year: u16) -> Vec<Date> {
        self.months_all()
            .into_iter()
            .filter(|month| month.year() == year)
            .collect()
    }

    /// The versions filed under `month`, newest first. Row three.
    pub fn versions(self, month: Date) -> Vec<&'static str> {
        self.all()
            .filter(|release| self.month_of(release) == Some(month))
            .map(|release| release.version)
            .collect()
    }

    /// The chips `selection` lights: its own and every level above it.
    pub fn path(self, selection: Selection) -> Path {
        match selection {
            Selection::Latest => Path::default(),
            Selection::Year(year) => Path {
                year: Some(year),
                ..Path::default()
            },
            Selection::Month(month) => Path {
                year: Some(month.year()),
                month: Some(month),
                version: None,
            },
            Selection::Version(version) => {
                let month = self.find(version).and_then(|r| self.month_of(r));
                Path {
                    year: month.map(Date::year),
                    month,
                    version: Some(version),
                }
            }
        }
    }

    /// One level up from `selection`: version to its month, month to its
    /// year, year to Latest.
    pub fn parent(self, selection: Selection) -> Selection {
        let path = self.path(selection);
        match selection {
            Selection::Latest | Selection::Year(_) => Selection::Latest,
            Selection::Month(month) => Selection::Year(month.year()),
            Selection::Version(_) => path.month.map_or(Selection::Latest, Selection::Month),
        }
    }

    /// The selection after a tap on `chip`. A chip above the current pick
    /// selects its level and clears the rows below it; tapping the chip
    /// already picked steps back up one level.
    pub fn tap(self, current: Selection, chip: Selection) -> Selection {
        if chip == current {
            self.parent(current)
        } else {
            chip
        }
    }

    /// The releases `selection` shows, in page order.
    pub fn shown(self, selection: Selection) -> Vec<&'static Release> {
        self.all()
            .filter(|release| match selection {
                Selection::Latest => {
                    self.is_upcoming(release.version) || self.is_latest(release.version)
                }
                Selection::Year(year) => self.month_of(release).map(Date::year) == Some(year),
                Selection::Month(month) => self.month_of(release) == Some(month),
                Selection::Version(version) => release.version == version,
            })
            .collect()
    }

    /// The selection a link to `fragment` opens on, with or without its
    /// leading `#`: the release whose [`anchor`] it is. `None` when no
    /// release has that anchor, so the page stays on Latest.
    pub fn for_anchor(self, fragment: &str) -> Option<Selection> {
        let fragment = fragment.strip_prefix('#').unwrap_or(fragment);
        self.all()
            .find(|release| anchor(release.version) == fragment)
            .map(|release| Selection::Version(release.version))
    }
}

/// A release card's id and the fragment that links to it: `v` and the
/// version with its dots as dashes, e.g. `"v1-1-0"` for 1.1.0.
pub fn anchor(version: &str) -> String {
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
    let path = log.path(selection);
    let mut tap = move |chip: Selection| selected.set(log.tap(selected(), chip));
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
                    for version in log.versions(month) {
                        Chip {
                            key: "{version}",
                            selected: path.version == Some(version),
                            onclick: move |_| tap(Selection::Version(version)),
                            "{version}"
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

    fn versions(releases: &[&Release]) -> Vec<&'static str> {
        releases.iter().map(|r| r.version).collect()
    }

    /// Releases across a new year, with an upcoming version: 2.1.0 is
    /// upcoming, 2.0.0 shipped in January 2027, the rest in late 2026.
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
        assert_eq!(TWO_YEARS.path(Selection::Latest), Path::default());
    }

    /// The year row shows even when every release is in one year.
    #[test]
    fn one_year_still_gets_a_year_chip() {
        assert_eq!(Log::CAIRN.years(), [2026]);
        assert_eq!(Log::CAIRN.months(2026), [OCT]);
        assert_eq!(Log::CAIRN.versions(OCT), ["1.1.1", "1.1.0", "1.0.0"]);
        assert_eq!(
            versions(&Log::CAIRN.shown(Selection::Year(2026))),
            ["1.1.1", "1.1.0", "1.0.0"]
        );
    }

    #[test]
    fn each_level_narrows_the_cards() {
        let log = TWO_YEARS;
        let dec = Date::new(2026, 12, 1);
        let jan = Date::new(2027, 1, 1);
        assert_eq!(log.years(), [2027, 2026]);
        assert_eq!(log.months(2026), [dec, Date::new(2026, 11, 1)]);
        assert_eq!(log.months(2027), [jan]);
        // The upcoming version files under the newest month.
        assert_eq!(log.versions(jan), ["2.1.0", "2.0.0"]);
        assert_eq!(log.versions(dec), ["1.3.0", "1.2.0"]);

        assert_eq!(
            versions(&log.shown(Selection::Year(2026))),
            ["1.3.0", "1.2.0", "1.1.0"]
        );
        assert_eq!(
            versions(&log.shown(Selection::Month(dec))),
            ["1.3.0", "1.2.0"]
        );
        assert_eq!(versions(&log.shown(Selection::Version("1.2.0"))), ["1.2.0"]);
        assert_eq!(
            log.path(Selection::Version("1.2.0")),
            Path {
                year: Some(2026),
                month: Some(dec),
                version: Some("1.2.0"),
            }
        );
        assert_eq!(
            log.path(Selection::Month(jan)),
            Path {
                year: Some(2027),
                month: Some(jan),
                version: None,
            }
        );
    }

    #[test]
    fn a_higher_chip_clears_the_levels_below() {
        let log = TWO_YEARS;
        let dec = Date::new(2026, 12, 1);
        let picked = Selection::Version("1.2.0");
        assert_eq!(
            log.tap(picked, Selection::Month(dec)),
            Selection::Month(dec)
        );
        assert_eq!(
            log.tap(picked, Selection::Year(2026)),
            Selection::Year(2026)
        );
        assert_eq!(
            log.tap(picked, Selection::Year(2027)),
            Selection::Year(2027)
        );
        assert_eq!(log.tap(picked, Selection::Latest), Selection::Latest);
    }

    #[test]
    fn tapping_the_picked_chip_steps_up_one_level() {
        let log = TWO_YEARS;
        let dec = Date::new(2026, 12, 1);
        let version = Selection::Version("1.2.0");
        assert_eq!(log.tap(version, version), Selection::Month(dec));
        assert_eq!(
            log.tap(Selection::Month(dec), Selection::Month(dec)),
            Selection::Year(2026)
        );
        assert_eq!(
            log.tap(Selection::Year(2026), Selection::Year(2026)),
            Selection::Latest
        );
        assert_eq!(
            log.tap(Selection::Latest, Selection::Latest),
            Selection::Latest
        );
    }

    #[test]
    fn anchors_are_the_version_with_dashes() {
        assert_eq!(anchor("1.1.0"), "v1-1-0");
        assert_eq!(anchor("10.0.12"), "v10-0-12");
    }

    #[test]
    fn an_anchor_picks_its_year_month_and_version() {
        let picked = Log::CAIRN.for_anchor("#v1-0-0");
        assert_eq!(picked, Some(Selection::Version("1.0.0")));
        assert_eq!(
            Log::CAIRN.path(Selection::Version("1.0.0")),
            Path {
                year: Some(2026),
                month: Some(OCT),
                version: Some("1.0.0"),
            }
        );
        assert_eq!(
            Log::CAIRN.for_anchor("v1-1-1"),
            Some(Selection::Version("1.1.1"))
        );
        assert_eq!(
            TWO_YEARS.path(TWO_YEARS.for_anchor("#v2-1-0").unwrap_or(Selection::Latest)),
            Path {
                year: Some(2027),
                month: Some(Date::new(2027, 1, 1)),
                version: Some("2.1.0"),
            }
        );
    }

    #[test]
    fn an_unknown_anchor_stays_on_latest() {
        for fragment in ["", "#", "#v9-9-9", "#1.0.0", "#top"] {
            assert_eq!(Log::CAIRN.for_anchor(fragment), None, "{fragment}");
        }
    }
}
