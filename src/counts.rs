//! Line, test and lint counts for each repository, measured from a local clone.
//!
//! `counts.json` holds the numbers the site shows. `measure` produces them from a
//! checkout, and the `counts_match_the_local_clones` test compares the two when
//! `PORTFOLIO_REPOS` names the directory the clones live in. With
//! `PORTFOLIO_WRITE_COUNTS=1` that test rewrites the file instead of failing, which
//! is how the numbers are refreshed.

use crate::stats::with_separators;
use serde::{Deserialize, Serialize};
use std::{collections::BTreeMap, fmt::Write, sync::LazyLock};

const FILE: &str = include_str!("counts.json");

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum Language {
    Rust,
    CSharp,
}

impl Language {
    fn name(self) -> &'static str {
        match self {
            Self::Rust => "Rust",
            Self::CSharp => "C#",
        }
    }
}

/// What one repository's source contains.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Counts {
    pub language: Language,
    /// Lines in every source file of the language, `target/` and the like excluded.
    pub lines: u64,
    /// Test functions, counted by their attribute.
    pub tests: u64,
    /// Clippy lints configured at `warn` or `deny` in `Cargo.toml`, which CI runs
    /// with `-D warnings`. `None` for a repository with no `Cargo.toml`.
    pub clippy_lints: Option<u64>,
}

impl Counts {
    /// One line for a card: `6,102 lines of Rust, 245 tests, 13 clippy lints`.
    /// A count of zero is left out.
    pub fn line(&self) -> String {
        let mut line = format!(
            "{} lines of {}",
            with_separators(self.lines),
            self.language.name()
        );
        if self.tests > 0 {
            let unit = if self.tests == 1 { "test" } else { "tests" };
            write!(line, ", {} {unit}", with_separators(self.tests)).ok();
        }
        if let Some(lints) = self.clippy_lints.filter(|lints| *lints > 0) {
            let unit = if lints == 1 { "lint" } else { "lints" };
            write!(line, ", {} clippy {unit}", with_separators(lints)).ok();
        }
        line
    }
}

/// Keyed by `owner/name`. Empty when the file does not parse, and the site then
/// renders without these lines.
static COUNTS: LazyLock<BTreeMap<String, Counts>> =
    LazyLock::new(|| serde_json::from_str(FILE).unwrap_or_default());

fn key(repo_url: &str) -> String {
    repo_url
        .trim_start_matches("https://github.com/")
        .trim_end_matches('/')
        .to_ascii_lowercase()
}

/// The counts for a repository, by its GitHub link.
pub fn for_repo(repo_url: &str) -> Option<&'static Counts> {
    COUNTS.get(&key(repo_url))
}

/// One line for a card, when the repository has been measured.
pub fn line_for(repo_url: &str) -> Option<String> {
    for_repo(repo_url).map(Counts::line)
}

/// Only a test reads a checkout, so the site's build carries none of this.
#[cfg(test)]
pub use measure::measure;

#[cfg(test)]
mod measure {
    use super::{Counts, Language};
    use std::{fs, io, path::Path};

    impl Language {
        fn extension(self) -> &'static str {
            match self {
                Self::Rust => "rs",
                Self::CSharp => "cs",
            }
        }

        /// True when a trimmed source line is a test attribute.
        fn is_test_attribute(self, line: &str) -> bool {
            match self {
                Self::Rust => {
                    line == "#[test]"
                        || [
                            "#[tokio::test",
                            "#[rstest",
                            "#[wasm_bindgen_test",
                            "#[sqlx::test",
                        ]
                        .iter()
                        .any(|prefix| line.starts_with(prefix))
                }
                Self::CSharp => ["[Fact", "[Theory", "[Test]", "[TestMethod"]
                    .iter()
                    .any(|prefix| line.starts_with(prefix)),
            }
        }
    }

    /// Directories that hold build output or dependencies, never source.
    const SKIPPED: &[&str] = &["target", "bin", "obj", "node_modules"];

    /// Measures a checkout. `None` when it is neither a Rust nor a C# repository.
    pub fn measure(repo: &Path) -> io::Result<Option<Counts>> {
        let Some(language) = language_of(repo)? else {
            return Ok(None);
        };
        let mut lines = 0;
        let mut tests = 0;
        walk(repo, language, &mut |source| {
            for line in source.lines() {
                lines += 1;
                if language.is_test_attribute(line.trim()) {
                    tests += 1;
                }
            }
        })?;
        let clippy_lints = match language {
            Language::Rust => Some(clippy_lints(&fs::read_to_string(repo.join("Cargo.toml"))?)),
            Language::CSharp => None,
        };
        Ok(Some(Counts {
            language,
            lines,
            tests,
            clippy_lints,
        }))
    }

    /// Rust when there is a `Cargo.toml` at the root, C# when there is a project or
    /// solution file.
    fn language_of(repo: &Path) -> io::Result<Option<Language>> {
        if repo.join("Cargo.toml").is_file() {
            return Ok(Some(Language::Rust));
        }
        for entry in fs::read_dir(repo)? {
            let path = entry?.path();
            if matches!(
                path.extension().and_then(|e| e.to_str()),
                Some("csproj" | "sln" | "slnx")
            ) {
                return Ok(Some(Language::CSharp));
            }
        }
        Ok(None)
    }

    /// Calls `visit` with the text of every source file under `dir`, skipping
    /// hidden directories and the ones in `SKIPPED`.
    fn walk(dir: &Path, language: Language, visit: &mut dyn FnMut(&str)) -> io::Result<()> {
        let mut entries: Vec<_> = fs::read_dir(dir)?.collect::<Result<_, _>>()?;
        entries.sort_by_key(fs::DirEntry::path);
        for entry in entries {
            let path = entry.path();
            let name = entry.file_name();
            let name = name.to_string_lossy();
            if path.is_dir() {
                if !name.starts_with('.') && !SKIPPED.contains(&name.as_ref()) {
                    walk(&path, language, visit)?;
                }
            } else if path.extension().and_then(|e| e.to_str()) == Some(language.extension()) {
                visit(&fs::read_to_string(&path)?);
            }
        }
        Ok(())
    }

    /// Entries under `[lints.clippy]` or `[workspace.lints.clippy]` whose level is
    /// `warn` or `deny`, in either the `"deny"` or the `{ level = "deny" }` form.
    fn clippy_lints(cargo_toml: &str) -> u64 {
        let mut in_clippy = false;
        let mut lints = 0;
        for line in cargo_toml.lines().map(str::trim) {
            if line.starts_with('[') {
                in_clippy = matches!(line, "[lints.clippy]" | "[workspace.lints.clippy]");
            } else if in_clippy && !line.starts_with('#') {
                let Some((_, value)) = line.split_once('=') else {
                    continue;
                };
                let value = value.trim();
                if ["deny", "warn"].iter().any(|level| {
                    value == format!("\"{level}\"")
                        || value.contains(&format!("level = \"{level}\""))
                }) {
                    lints += 1;
                }
            }
        }
        lints
    }

    #[cfg(test)]
    mod tests {
        use super::*;

        #[test]
        fn clippy_lints_counts_warn_and_deny_in_a_clippy_section() {
            let toml = r#"
[lints.rust]
unsafe_code = "deny"

[lints.clippy]
pedantic = { level = "warn", priority = -1 }
# a comment with "deny" in it
todo = "deny"
unwrap_used = { level = "deny", priority = 1 }
as_conversions = "warn"
must_use_candidate = "allow"

[dependencies]
serde = "deny"
"#;
            assert_eq!(clippy_lints(toml), 4);
        }

        #[test]
        fn clippy_lints_reads_a_workspace_section() {
            assert_eq!(
                clippy_lints("[workspace.lints.clippy]\npanic = \"deny\"\nexit = \"warn\"\n"),
                2
            );
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::data::{Project, featured_projects, side_quests};
    use std::{fs, path::PathBuf};

    fn counts(lines: u64, tests: u64, clippy_lints: Option<u64>) -> Counts {
        Counts {
            language: Language::Rust,
            lines,
            tests,
            clippy_lints,
        }
    }

    #[test]
    fn the_file_parses() {
        let parsed = serde_json::from_str::<BTreeMap<String, Counts>>(FILE);
        assert!(parsed.is_ok(), "{:?}", parsed.err());
    }

    #[test]
    fn a_line_reads_as_a_sentence() {
        assert_eq!(
            counts(6102, 245, Some(13)).line(),
            "6,102 lines of Rust, 245 tests, 13 clippy lints"
        );
        assert_eq!(
            counts(1, 1, Some(1)).line(),
            "1 lines of Rust, 1 test, 1 clippy lint"
        );
        assert_eq!(counts(155, 0, Some(0)).line(), "155 lines of Rust");
        let csharp = Counts {
            language: Language::CSharp,
            ..counts(2600, 62, None)
        };
        assert_eq!(csharp.line(), "2,600 lines of C#, 62 tests");
    }

    #[test]
    fn a_repository_is_found_whatever_the_link_looks_like() {
        let expected = for_repo("https://github.com/scadoshi/steller");
        assert!(expected.is_some());
        for url in [
            "https://github.com/scadoshi/steller/",
            "https://github.com/Scadoshi/Steller",
            "scadoshi/steller",
        ] {
            assert_eq!(for_repo(url), expected, "{url}");
        }
    }

    fn every_project() -> impl Iterator<Item = &'static Project> {
        featured_projects().iter().chain(side_quests())
    }

    #[test]
    fn every_project_on_the_site_is_counted() {
        let missing: Vec<&str> = every_project()
            .filter(|project| for_repo(project.repo_url).is_none())
            .map(|project| project.repo_url)
            .collect();
        assert!(missing.is_empty(), "not in counts.json: {missing:?}");
    }

    /// The counts come from `counts.json`, so a number typed into the copy would
    /// drift from it. The narrative fields (`approach`, `obstacles`, snippets) are
    /// free to tell a story with a number in it.
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
        for project in every_project() {
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
            for bullet in project.card_bullets {
                if is_count(bullet) {
                    typed.push(format!("{}.card_bullets: {bullet}", project.slug));
                }
            }
        }
        assert!(typed.is_empty(), "counts typed by hand: {typed:#?}");
    }

    /// Set `PORTFOLIO_REPOS` to the directory the clones live in to run this. It
    /// skips repositories not cloned there and names them. With
    /// `PORTFOLIO_WRITE_COUNTS=1` it rewrites `counts.json` instead of failing.
    #[test]
    fn counts_match_the_local_clones() {
        let Some(root) = std::env::var_os("PORTFOLIO_REPOS") else {
            eprintln!("PORTFOLIO_REPOS unset; not checking counts.json against clones");
            return;
        };
        let root = PathBuf::from(root);
        let mut measured = COUNTS.clone();
        let mut drift = Vec::new();
        for project in every_project() {
            let name = project.repo_url.rsplit('/').next().unwrap_or_default();
            let repo = root.join(name);
            if !repo.is_dir() {
                eprintln!("{name}: not cloned under {}", root.display());
                continue;
            }
            let Some(fresh) = measure(&repo).expect("readable checkout") else {
                eprintln!("{name}: neither Rust nor C#");
                continue;
            };
            if measured.get(&key(project.repo_url)) != Some(&fresh) {
                drift.push(format!("{name}: {fresh:?}"));
                measured.insert(key(project.repo_url), fresh);
            }
        }
        if drift.is_empty() {
            return;
        }
        if std::env::var_os("PORTFOLIO_WRITE_COUNTS").is_some() {
            let json = serde_json::to_string_pretty(&measured).expect("serializes");
            fs::write(
                concat!(env!("CARGO_MANIFEST_DIR"), "/src/counts.json"),
                json + "\n",
            )
            .expect("counts.json is writable");
            eprintln!("counts.json rewritten: {}", drift.join(", "));
            return;
        }
        panic!(
            "counts.json is behind the clones (PORTFOLIO_WRITE_COUNTS=1 rewrites it): {drift:#?}"
        );
    }
}
