//! GitHub numbers for each project, served by heron
//! (<https://github.com/scadoshi/heron>).
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

/// Sums across every repository in the snapshot.
#[derive(Debug, Clone, PartialEq, Eq, Deserialize)]
pub struct Totals {
    pub repos: u32,
    pub commits: u64,
    pub stars: u64,
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
}

/// The body of heron's `GET /stats`.
#[derive(Debug, Clone, PartialEq, Eq, Deserialize)]
pub struct Snapshot {
    /// RFC 3339 in UTC.
    pub generated_at: String,
    pub totals: Totals,
    pub repos: Vec<RepoStats>,
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
fn current(live: Option<&Snapshot>) -> Option<(&Snapshot, Source)> {
    if let Some(snapshot) = live {
        return Some((snapshot, Source::Live));
    }
    BAKED
        .as_ref()
        .map(|baked| (baked, Source::AsOf(baked.day().to_string())))
}

/// One line for a card: `2,968 commits, last push 2026-09-29`.
pub fn line_for(live: Option<&Snapshot>, repo_url: &str) -> Option<String> {
    let (snapshot, _) = current(live)?;
    snapshot.repo(repo_url).map(RepoStats::line)
}

/// The totals and where they came from.
pub fn totals(live: Option<&Snapshot>) -> Option<(Totals, Source)> {
    current(live).map(|(snapshot, source)| (snapshot.totals.clone(), source))
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
    /// One line for a card: `2,968 commits, last push 2026-09-29`.
    pub fn line(&self) -> String {
        let commits = format!(
            "{} {}",
            with_separators(self.commits),
            if self.commits == 1 {
                "commit"
            } else {
                "commits"
            }
        );
        // The date is the first ten characters of an RFC 3339 timestamp.
        match self.pushed_at.as_deref().and_then(|at| at.get(..10)) {
            Some(date) => format!("{commits}, last push {date}"),
            None => commits,
        }
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
            line_for(None, "https://github.com/scadoshi/steller"),
            baked()
                .repo("https://github.com/scadoshi/steller")
                .map(RepoStats::line)
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
            },
            repos: vec![RepoStats {
                repo: "scadoshi/steller".to_string(),
                commits: 4242,
                pushed_at: Some("2026-10-01T08:59:00Z".to_string()),
            }],
        };
        let (totals, source) = totals(Some(&live)).expect("live totals");
        assert_eq!(totals.commits, 4242);
        assert_eq!(source, Source::Live);
        assert_eq!(
            line_for(Some(&live), "https://github.com/scadoshi/steller").as_deref(),
            Some("4,242 commits, last push 2026-10-01")
        );
        // A repository the live answer lacks is absent, not filled from the build.
        assert_eq!(
            line_for(Some(&live), "https://github.com/scadoshi/zwipe"),
            None
        );
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
    fn the_line_names_commits_and_the_day_of_the_last_push() {
        let stats = |commits, pushed_at: Option<&str>| RepoStats {
            repo: "a/b".to_string(),
            commits,
            pushed_at: pushed_at.map(ToString::to_string),
        };
        assert_eq!(
            stats(2968, Some("2026-09-29T12:13:17Z")).line(),
            "2,968 commits, last push 2026-09-29"
        );
        assert_eq!(stats(1, None).line(), "1 commit");
        assert_eq!(stats(12, Some("short")).line(), "12 commits");
    }
}
