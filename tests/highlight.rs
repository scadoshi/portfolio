//! The build-time highlighter: which snippets it finds in `src/data.rs`, what
//! it tags them with, and that every tag it emits is styled.

#[path = "../build/highlight.rs"]
mod highlight;

use highlight::{highlight, snippets};

const DATA: &str = include_str!("../src/data.rs");
const CSS: &str = include_str!("../assets/main.css");

/// Classes left the text color on purpose: plain variables and module paths
/// read as text in the editor too.
const PLAIN: [&str; 3] = ["l-variable", "l-module", "l-namespace"];

fn rust(code: &str) -> String {
    highlight("rust", code).expect("rust highlights")
}

/// The text the reader sees: tags dropped, entities decoded.
fn visible_text(html: &str) -> String {
    let mut text = String::new();
    let mut in_tag = false;
    for c in html.chars() {
        match c {
            '<' => in_tag = true,
            '>' if in_tag => in_tag = false,
            _ if !in_tag => text.push(c),
            _ => {}
        }
    }
    text.replace("&lt;", "<")
        .replace("&gt;", ">")
        .replace("&quot;", "\"")
        .replace("&#39;", "'")
        .replace("&#x27;", "'")
        .replace("&amp;", "&")
}

/// Every `l-*` class in `html`.
fn classes(html: &str) -> Vec<String> {
    html.split("class=\"")
        .skip(1)
        .filter_map(|rest| rest.split_once('"').map(|(class, _)| class.to_string()))
        .collect()
}

#[test]
fn finds_every_snippet_in_the_data_file() {
    let found = snippets(DATA).expect("data.rs parses");
    let literals = DATA.matches("Snippet {").count() - DATA.matches("pub struct Snippet {").count();
    assert_eq!(found.len(), literals);
    assert!(found.iter().all(|s| !s.code.trim().is_empty()));
}

#[test]
fn reads_raw_strings_and_skips_other_structs() {
    let source = r##"
        const A: Snippet = Snippet { title: "a", lang: "rust", code: r#"let s = "quoted";"#, description: "" };
        const B: Other = Other { lang: "rust", code: "not a snippet" };
        const C: &[Snippet] = &[Snippet { title: "c", lang: "csharp", code: "var x = 1;", description: "" }];
    "##;
    let found = snippets(source).expect("parses");
    let codes: Vec<&str> = found.iter().map(|s| s.code.as_str()).collect();
    assert_eq!(codes, ["let s = \"quoted\";", "var x = 1;"]);
    assert_eq!(found[1].lang, "csharp");
}

#[test]
fn tags_what_an_editor_tags() {
    let html = rust(
        "fn solver_for(year: i32, day: i32) -> Option<Solver> {\n    Some(match (year, day) {\n        (2015, 1) => solve::<year_2015::day_01::Puzzle>,\n        _ => return None,\n    })\n}",
    );
    for expected in [
        r#"<span class="l-keyword-function">fn</span>"#,
        r#"<span class="l-function">solver_for</span>"#,
        r#"<span class="l-variable-parameter">year</span>"#,
        r#"<span class="l-function-call">solve</span>"#,
        r#"<span class="l-punctuation-delimiter">=&gt;</span>"#,
        r#"<span class="l-punctuation-delimiter">::</span>"#,
        r#"<span class="l-type">Puzzle</span>"#,
        r#"<span class="l-constant-builtin">Some</span>"#,
        r#"<span class="l-number">2015</span>"#,
    ] {
        assert!(html.contains(expected), "missing {expected} in\n{html}");
    }
}

#[test]
fn tags_method_calls_and_fields() {
    let html = rust("let v = outcome.value(); self.count += 1;");
    assert!(
        html.contains(r#"<span class="l-function-call">value</span>"#),
        "{html}"
    );
    assert!(
        html.contains(r#"<span class="l-variable-member">count</span>"#),
        "{html}"
    );
}

#[test]
fn every_macro_gets_the_macro_class() {
    let html = rust(r#"fn f() { eprintln!("x"); panic!("y"); println!("z"); matches!(a, b); }"#);
    for name in ["eprintln!", "panic!", "println!", "matches!"] {
        let tagged = format!(r#"<span class="l-function-macro">{name}</span>"#);
        assert!(html.contains(&tagged), "{name} not a macro in\n{html}");
    }
}

#[test]
fn highlights_csharp() {
    let html = highlight(
        "csharp",
        "public async Task<int> Run(string name) { return await Foo.Bar(name); }",
    )
    .expect("csharp highlights");
    assert!(
        html.contains(r#"<span class="l-keyword-modifier">public</span>"#),
        "{html}"
    );
    assert!(
        html.contains(r#"<span class="l-variable-parameter">name</span>"#),
        "{html}"
    );
    assert!(
        html.contains(r#"<span class="l-function-method-call">Bar</span>"#),
        "{html}"
    );
}

#[test]
fn an_unknown_language_is_an_error() {
    assert!(highlight("cobol", "DISPLAY 'HI'.").is_err());
}

#[test]
fn leaves_no_wrapper_markup() {
    let html = rust("fn a() {}\nfn b() {}");
    assert!(
        !html.contains("<pre") && !html.contains("<code") && !html.contains("l-line"),
        "{html}"
    );
    assert_eq!(html.lines().count(), 2);
}

#[test]
fn every_snippet_reads_back_exactly_as_written() {
    for s in snippets(DATA).expect("data.rs parses") {
        let code = s.code.trim();
        let html = highlight(&s.lang, code).expect("highlights");
        assert_eq!(
            visible_text(&html),
            code,
            "text changed in a {} snippet",
            s.lang
        );
    }
}

#[test]
fn every_class_the_snippets_use_is_styled() {
    let mut unstyled = Vec::new();
    for s in snippets(DATA).expect("data.rs parses") {
        let html = highlight(&s.lang, s.code.trim()).expect("highlights");
        for class in classes(&html) {
            let styled = CSS.contains(&format!(".code-content .{class},"))
                || CSS.contains(&format!(".code-content .{class} {{"));
            if !styled && !PLAIN.contains(&class.as_str()) && !unstyled.contains(&class) {
                unstyled.push(class);
            }
        }
    }
    assert!(
        unstyled.is_empty(),
        "add these to main.css or PLAIN: {unstyled:?}"
    );
}
