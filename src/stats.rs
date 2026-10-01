//! The numbers for each project, served by heron
//! (<https://github.com/scadoshi/heron>): commits and the last push from GitHub,
//! lines, tests and clippy lints measured by heron from each repository's default
//! branch.
//!
//! Two sources, in order of preference. The page asks heron after it loads, and
//! shows that answer as live. Until it arrives, or if it never does, the page shows
//! `stats.json`, the answer heron gave when the site was built. The deploy workflow
//! replaces that file before each build and keeps the committed copy when heron
//! cannot give a complete one, so a build never waits on heron and a visitor never
//! sees an empty number.

use dioxus::prelude::*;
use serde::Deserialize;
use std::sync::LazyLock;

const SNAPSHOT: &str = include_str!("stats.json");

/// Commits in one week.
#[derive(Debug, Clone, PartialEq, Eq, Deserialize)]
pub struct WeekCommits {
    /// The Sunday the week starts on, `YYYY-MM-DD`.
    pub week: String,
    pub commits: u32,
}

/// Sums across every repository in the snapshot.
#[derive(Debug, Clone, PartialEq, Eq, Deserialize)]
pub struct Totals {
    pub repos: u32,
    pub commits: u64,
    pub stars: u64,
    /// Commits per week across the repositories, oldest first. Empty in an
    /// answer from before heron served it.
    #[serde(default)]
    pub weekly_commits: Vec<WeekCommits>,
}

/// What heron counted in a repository's source. `language` is the name as heron
/// prints it, `Rust` or `C#`.
#[derive(Debug, Clone, PartialEq, Eq, Deserialize)]
pub struct Counts {
    pub language: String,
    /// Lines in every source file, blanks and comments included.
    pub lines: u64,
    /// Test attributes, one per test function.
    pub tests: u64,
    /// Clippy lints set to warn or deny. `None` for C#.
    pub clippy_lints: Option<u64>,
    /// RFC 3339 in UTC, when heron measured the source.
    pub measured_at: String,
}

impl Counts {
    /// Chips for a card: `6,102 lines of Rust`, `245 tests`, `13 clippy lints`. A
    /// count of zero is left out.
    pub fn chips(&self) -> Vec<String> {
        let mut chips = vec![format!(
            "{} lines of {}",
            with_separators(self.lines),
            self.language
        )];
        if self.tests > 0 {
            let unit = if self.tests == 1 { "test" } else { "tests" };
            chips.push(format!("{} {unit}", with_separators(self.tests)));
        }
        if let Some(lints) = self.clippy_lints.filter(|lints| *lints > 0) {
            let unit = if lints == 1 { "lint" } else { "lints" };
            chips.push(format!("{} clippy {unit}", with_separators(lints)));
        }
        chips
    }
}

/// What heron reports for one repository. Only the fields the site shows.
#[derive(Debug, Clone, PartialEq, Eq, Deserialize)]
pub struct RepoStats {
    /// `owner/name`.
    pub repo: String,
    /// Commits reachable from the default branch.
    pub commits: u64,
    /// RFC 3339 in UTC. `None` for a repository never pushed to.
    pub pushed_at: Option<String>,
    /// `None` until heron's sweep has measured the repository, and in a snapshot
    /// from before heron measured anything.
    #[serde(default)]
    pub counts: Option<Counts>,
}

/// One day of the contribution calendar.
#[derive(Debug, Clone, PartialEq, Eq, Deserialize)]
pub struct Day {
    /// `YYYY-MM-DD`.
    pub date: String,
    pub count: u32,
    /// 0 for none through 4 for the top quartile, GitHub's own shading.
    pub level: u8,
}

/// A year of contributions on GitHub across every repository, as the profile
/// page draws it.
#[derive(Debug, Clone, PartialEq, Eq, Deserialize)]
pub struct Calendar {
    pub login: String,
    pub total: u32,
    /// Oldest first, without gaps.
    pub days: Vec<Day>,
}

/// The body of heron's `GET /stats`.
#[derive(Debug, Clone, PartialEq, Eq, Deserialize)]
pub struct Snapshot {
    /// RFC 3339 in UTC.
    pub generated_at: String,
    pub totals: Totals,
    pub repos: Vec<RepoStats>,
    /// `None` in an answer from before heron served it, or when heron could
    /// not read it.
    #[serde(default)]
    pub calendar: Option<Calendar>,
}

impl Snapshot {
    fn repo(&self, repo_url: &str) -> Option<&RepoStats> {
        let repo = repo_url
            .trim_start_matches("https://github.com/")
            .trim_end_matches('/');
        self.repos
            .iter()
            .find(|stats| stats.repo.eq_ignore_ascii_case(repo))
    }

    /// The day the numbers were assembled: the first ten characters of the timestamp.
    fn day(&self) -> &str {
        self.generated_at.get(..10).unwrap_or_default()
    }
}

/// `None` when the snapshot does not parse. The site then renders without numbers.
static BAKED: LazyLock<Option<Snapshot>> = LazyLock::new(|| serde_json::from_str(SNAPSHOT).ok());

/// Heron's answer after the page loaded, shared through context. `None` until it
/// arrives, and forever if it never does.
pub type Live = Signal<Option<Snapshot>>;

/// Which source a number came from.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Source {
    /// From heron, just now.
    Live,
    /// From the build, on this day.
    AsOf(String),
}

/// The live snapshot when there is one, else the baked one.
pub fn current(live: Option<&Snapshot>) -> Option<(&Snapshot, Source)> {
    if let Some(snapshot) = live {
        return Some((snapshot, Source::Live));
    }
    BAKED
        .as_ref()
        .map(|baked| (baked, Source::AsOf(baked.day().to_string())))
}

/// Chips for a card, when the repository is in the current snapshot.
pub fn chips_for(live: Option<&Snapshot>, repo_url: &str) -> Option<Vec<String>> {
    let (snapshot, _) = current(live)?;
    snapshot.repo(repo_url).map(RepoStats::chips)
}

/// The totals and where they came from.
pub fn totals(live: Option<&Snapshot>) -> Option<(Totals, Source)> {
    current(live).map(|(snapshot, source)| (snapshot.totals.clone(), source))
}

/// Commits per week across every repository, oldest first, and where they came
/// from. `None` when the current snapshot has none.
pub fn weekly_commits(live: Option<&Snapshot>) -> Option<(&[WeekCommits], Source)> {
    let (snapshot, source) = current(live)?;
    if snapshot.totals.weekly_commits.is_empty() {
        return None;
    }
    Some((&snapshot.totals.weekly_commits, source))
}

/// The contribution calendar and where it came from, when the current snapshot
/// has one.
pub fn calendar(live: Option<&Snapshot>) -> Option<(&Calendar, Source)> {
    let (snapshot, source) = current(live)?;
    snapshot
        .calendar
        .as_ref()
        .map(|calendar| (calendar, source))
}

/// Line, test and lint chips for a card, when the current snapshot has measured
/// the repository.
pub fn count_chips_for(live: Option<&Snapshot>, repo_url: &str) -> Option<Vec<String>> {
    current(live)
        .and_then(|(snapshot, _)| snapshot.repo(repo_url))
        .and_then(|stats| stats.counts.as_ref())
        .map(Counts::chips)
}

/// Asks heron for the live numbers. `None` on any failure.
#[cfg(target_arch = "wasm32")]
pub async fn fetch_live() -> Option<Snapshot> {
    let response = gloo_net::http::Request::get("https://api.scadoshi.dev/stats")
        .send()
        .await
        .ok()?;
    if !response.ok() {
        return None;
    }
    response.json::<Snapshot>().await.ok()
}

/// Prerendering has no page to update, so there is nothing to ask for.
#[cfg(not(target_arch = "wasm32"))]
pub fn fetch_live() -> impl std::future::Future<Output = Option<Snapshot>> {
    std::future::ready(None)
}

impl RepoStats {
    /// Chips for a card: `2,968 commits` and `pushed 2026-09-29`.
    pub fn chips(&self) -> Vec<String> {
        let mut chips = vec![format!(
            "{} {}",
            with_separators(self.commits),
            if self.commits == 1 {
                "commit"
            } else {
                "commits"
            }
        )];
        // The date is the first ten characters of an RFC 3339 timestamp.
        if let Some(date) = self.pushed_at.as_deref().and_then(|at| at.get(..10)) {
            chips.push(format!("pushed {date}"));
        }
        chips
    }
}

/// Groups digits in threes: `3534` becomes `3,534`.
pub fn with_separators(number: u64) -> String {
    let digits = number.to_string();
    let mut grouped = String::with_capacity(digits.len() + digits.len() / 3);
    for (position, digit) in digits.chars().enumerate() {
        if position > 0 && (digits.len() - position).is_multiple_of(3) {
            grouped.push(',');
        }
        grouped.push(digit);
    }
    grouped
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::data::{featured_projects, side_quests};

    fn baked() -> &'static Snapshot {
        BAKED.as_ref().expect("snapshot parses")
    }

    #[test]
    fn the_snapshot_parses() {
        let snapshot = serde_json::from_str::<Snapshot>(SNAPSHOT);
        assert!(snapshot.is_ok(), "{:?}", snapshot.err());
    }

    /// A project added to `data.rs` also has to be added to `GITHUB_REPOS` on
    /// heron. Until it is, its card has no numbers, and this says which one.
    #[test]
    fn every_project_on_the_site_is_in_the_snapshot() {
        let missing: Vec<&str> = featured_projects()
            .iter()
            .chain(side_quests())
            .filter(|project| baked().repo(project.repo_url).is_none())
            .map(|project| project.repo_url)
            .collect();
        assert!(missing.is_empty(), "not served by heron: {missing:?}");
    }

    #[test]
    fn totals_agree_with_the_repositories() {
        let snapshot = baked();
        let commits: u64 = snapshot.repos.iter().map(|repo| repo.commits).sum();
        assert_eq!(snapshot.totals.commits, commits);
        assert_eq!(
            Some(snapshot.totals.repos),
            u32::try_from(snapshot.repos.len()).ok()
        );
    }

    #[test]
    fn a_repository_is_found_whatever_the_link_looks_like() {
        let expected = baked()
            .repo("https://github.com/scadoshi/steller")
            .map(|s| &s.repo);
        assert!(expected.is_some());
        for url in [
            "https://github.com/scadoshi/steller/",
            "https://github.com/Scadoshi/Steller",
            "scadoshi/steller",
        ] {
            assert_eq!(baked().repo(url).map(|s| &s.repo), expected, "{url}");
        }
        assert!(
            baked()
                .repo("https://github.com/scadoshi/not-a-repo")
                .is_none()
        );
    }

    #[test]
    fn without_a_live_answer_the_baked_numbers_and_their_day_are_used() {
        let (totals, source) = totals(None).expect("baked totals");
        assert_eq!(totals, baked().totals);
        assert_eq!(source, Source::AsOf(baked().day().to_string()));
        assert_eq!(baked().day().len(), 10, "{:?}", baked().generated_at);
        assert_eq!(
            chips_for(None, "https://github.com/scadoshi/steller"),
            baked()
                .repo("https://github.com/scadoshi/steller")
                .map(RepoStats::chips)
        );
    }

    #[test]
    fn a_live_answer_replaces_the_baked_numbers() {
        let live = Snapshot {
            generated_at: "2026-10-01T09:00:00Z".to_string(),
            totals: Totals {
                repos: 1,
                commits: 4242,
                stars: 7,
                weekly_commits: Vec::new(),
            },
            repos: vec![RepoStats {
                repo: "scadoshi/steller".to_string(),
                commits: 4242,
                pushed_at: Some("2026-10-01T08:59:00Z".to_string()),
                counts: Some(Counts {
                    language: "Rust".to_string(),
                    lines: 6200,
                    tests: 250,
                    clippy_lints: Some(14),
                    measured_at: "2026-10-01T08:58:00Z".to_string(),
                }),
            }],
            calendar: None,
        };
        let (totals, source) = totals(Some(&live)).expect("live totals");
        assert_eq!(totals.commits, 4242);
        assert_eq!(source, Source::Live);
        assert_eq!(
            chips_for(Some(&live), "https://github.com/scadoshi/steller"),
            Some(vec![
                "4,242 commits".to_string(),
                "pushed 2026-10-01".to_string()
            ])
        );
        // A repository the live answer lacks is absent, not filled from the build.
        assert_eq!(
            chips_for(Some(&live), "https://github.com/scadoshi/zwipe"),
            None
        );
        // Measured live, the counts come from heron.
        assert_eq!(
            count_chips_for(Some(&live), "https://github.com/scadoshi/steller"),
            Some(vec![
                "6,200 lines of Rust".to_string(),
                "250 tests".to_string(),
                "14 clippy lints".to_string()
            ])
        );
    }

    #[test]
    fn counts_come_from_the_baked_snapshot_without_a_live_answer() {
        let steller = "https://github.com/scadoshi/steller";
        let baked_chips = baked()
            .repo(steller)
            .and_then(|stats| stats.counts.as_ref())
            .map(Counts::chips);
        assert!(baked_chips.is_some());
        assert_eq!(count_chips_for(None, steller), baked_chips);
        // A live answer that has not measured the repository yet shows no counts
        // rather than mixing in the build's.
        let mut live = baked().clone();
        for repo in &mut live.repos {
            repo.counts = None;
        }
        assert_eq!(count_chips_for(Some(&live), steller), None);
    }

    /// heron measures every repository within seconds of starting, and the deploy
    /// workflow only bakes an answer in which every repository is measured.
    #[test]
    fn every_project_on_the_site_is_measured_in_the_snapshot() {
        let unmeasured: Vec<&str> = featured_projects()
            .iter()
            .chain(side_quests())
            .filter(|project| {
                baked()
                    .repo(project.repo_url)
                    .is_none_or(|stats| stats.counts.is_none())
            })
            .map(|project| project.repo_url)
            .collect();
        assert!(
            unmeasured.is_empty(),
            "not measured by heron: {unmeasured:?}"
        );
    }

    #[test]
    fn count_chips_leave_out_a_zero() {
        let counts = |language: &str, lines, tests, clippy_lints| Counts {
            language: language.to_string(),
            lines,
            tests,
            clippy_lints,
            measured_at: "2026-10-01T14:14:01Z".to_string(),
        };
        assert_eq!(
            counts("Rust", 6102, 245, Some(13)).chips(),
            ["6,102 lines of Rust", "245 tests", "13 clippy lints"]
        );
        assert_eq!(
            counts("Rust", 1, 1, Some(1)).chips(),
            ["1 lines of Rust", "1 test", "1 clippy lint"]
        );
        assert_eq!(
            counts("Rust", 155, 0, Some(0)).chips(),
            ["155 lines of Rust"]
        );
        assert_eq!(
            counts("C#", 2600, 62, None).chips(),
            ["2,600 lines of C#", "62 tests"]
        );
    }

    /// The counts come from heron, so a number typed into the copy would drift
    /// from them. `obstacles` and the snippets are free to tell a story with a
    /// number in it, since those are about a moment rather than the repo now.
    #[test]
    fn no_summary_field_types_a_count_by_hand() {
        let is_count = |text: &str| {
            text.split(|c: char| !c.is_alphanumeric() && c != ',' && c != '~' && c != '+')
                .collect::<Vec<_>>()
                .windows(2)
                .any(|pair| {
                    let number = pair[0].trim_start_matches('~').trim_end_matches('+');
                    let unit = pair[1].trim_end_matches(',');
                    !number.is_empty()
                        && number.chars().all(|c| c.is_ascii_digit() || c == ',')
                        && matches!(
                            unit,
                            "test" | "tests" | "line" | "lines" | "LOC" | "lint" | "lints"
                        )
                })
        };
        let mut typed = Vec::new();
        for project in featured_projects().iter().chain(side_quests()) {
            let fields = [
                ("headline", project.headline),
                ("summary", project.summary),
                ("impact_metric", project.impact_metric),
                ("objective", project.objective),
                ("progress", project.progress),
                ("impact", project.impact),
            ];
            for (field, text) in fields {
                if is_count(text) {
                    typed.push(format!("{}.{field}", project.slug));
                }
            }
            for bullet in project.card_bullets.iter().chain(project.approach) {
                if is_count(bullet) {
                    typed.push(format!("{}: {bullet}", project.slug));
                }
            }
        }
        assert!(typed.is_empty(), "counts typed by hand: {typed:#?}");
    }

    #[test]
    fn digits_are_grouped_in_threes() {
        for (number, grouped) in [
            (0, "0"),
            (7, "7"),
            (999, "999"),
            (1000, "1,000"),
            (3534, "3,534"),
            (100_000, "100,000"),
            (1_234_567, "1,234,567"),
        ] {
            assert_eq!(with_separators(number), grouped);
        }
    }

    #[test]
    fn the_chips_name_commits_and_the_day_of_the_last_push() {
        let stats = |commits, pushed_at: Option<&str>| RepoStats {
            repo: "a/b".to_string(),
            commits,
            pushed_at: pushed_at.map(ToString::to_string),
            counts: None,
        };
        assert_eq!(
            stats(2968, Some("2026-09-29T12:13:17Z")).chips(),
            ["2,968 commits", "pushed 2026-09-29"]
        );
        assert_eq!(stats(1, None).chips(), ["1 commit"]);
        assert_eq!(stats(12, Some("short")).chips(), ["12 commits"]);
    }
}
