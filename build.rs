//! Highlights the code snippets in `src/data.rs` and writes them to
//! `$OUT_DIR/highlighted.rs`, which `CodeBlock` includes.

use std::fmt::Write;

#[path = "build/highlight.rs"]
mod highlight;

fn main() {
    println!("cargo::rerun-if-changed=src/data.rs");
    println!("cargo::rerun-if-changed=build/highlight.rs");

    let source = std::fs::read_to_string("src/data.rs").expect("read src/data.rs");
    let snippets = highlight::snippets(&source).expect("parse src/data.rs");

    let mut out = String::from(
        "/// Each snippet's trimmed code and its highlighted HTML.\npub(crate) static HIGHLIGHTED: &[(&str, &str)] = &[\n",
    );
    for snippet in snippets {
        let code = snippet.code.trim();
        let html = highlight::highlight(&snippet.lang, code)
            .unwrap_or_else(|e| panic!("highlight a {} snippet: {e}", snippet.lang));
        writeln!(out, "    ({code:?}, {html:?}),").expect("write to a String");
    }
    out.push_str("];\n");

    let dir = std::env::var("OUT_DIR").expect("OUT_DIR is set for build scripts");
    std::fs::write(std::path::Path::new(&dir).join("highlighted.rs"), out)
        .expect("write highlighted.rs");
}
