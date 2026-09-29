//! GitHub numbers for each project, served by heron
//! (<https://github.com/scadoshi/heron>) and read when the site builds.
//!
//! `stats.json` is the body of `GET https://api.scadoshi.dev/stats`. The deploy
//! workflow replaces it with a new answer before each build and keeps the committed
//! copy when heron cannot give a complete one, so a build never waits on heron.

use serde::Deserialize;
use std::sync::LazyLock;

const SNAPSHOT: &str = include_str!("stats.json");

/// Sums across every repository in the snapshot.
#[derive(Debug, Deserialize)]
pub struct Totals {
    pub repos: u32,
    pub commits: u64,
}

/// What heron reports for one repository. Only the fields the site shows.
#[derive(Debug, Deserialize)]
pub struct RepoStats {
    /// `owner/name`.
    pub repo: String,
    /// Commits reachable from the default branch.
    pub commits: u64,
    /// RFC 3339 in UTC. `None` for a repository never pushed to.
    pub pushed_at: Option<String>,
}

#[derive(Debug, Deserialize)]
struct Snapshot {
    totals: Totals,
    repos: Vec<RepoStats>,
}

/// `None` when the snapshot does not parse. The site then renders without numbers.
static STATS: LazyLock<Option<Snapshot>> = LazyLock::new(|| serde_json::from_str(SNAPSHOT).ok());

/// Stats for the repository at `repo_url`, a `https://github.com/owner/name` link.
pub fn for_repo(repo_url: &str) -> Option<&'static RepoStats> {
    let repo = repo_url
        .trim_start_matches("https://github.com/")
        .trim_end_matches('/');
    STATS
        .as_ref()?
        .repos
        .iter()
        .find(|stats| stats.repo.eq_ignore_ascii_case(repo))
}

/// Sums across every repository in the snapshot.
pub fn totals() -> Option<&'static Totals> {
    STATS.as_ref().map(|snapshot| &snapshot.totals)
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
            .filter(|project| for_repo(project.repo_url).is_none())
            .map(|project| project.repo_url)
            .collect();
        assert!(missing.is_empty(), "not served by heron: {missing:?}");
    }

    #[test]
    fn totals_agree_with_the_repositories() {
        let snapshot: Snapshot = serde_json::from_str(SNAPSHOT).expect("snapshot parses");
        let commits: u64 = snapshot.repos.iter().map(|repo| repo.commits).sum();
        assert_eq!(snapshot.totals.commits, commits);
        assert_eq!(
            Some(snapshot.totals.repos),
            u32::try_from(snapshot.repos.len()).ok()
        );
    }

    #[test]
    fn a_repository_is_found_whatever_the_link_looks_like() {
        let expected = for_repo("https://github.com/scadoshi/steller").map(|s| &s.repo);
        assert!(expected.is_some());
        for url in [
            "https://github.com/scadoshi/steller/",
            "https://github.com/Scadoshi/Steller",
            "scadoshi/steller",
        ] {
            assert_eq!(for_repo(url).map(|s| &s.repo), expected, "{url}");
        }
        assert!(for_repo("https://github.com/scadoshi/not-a-repo").is_none());
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
