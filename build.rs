//! Highlights the code snippets in `src/data.rs` and writes them to
//! `$OUT_DIR/highlighted.rs`, which `CodeBlock` includes.

use std::fmt::Write;

#[path = "build/highlight.rs"]
mod highlight;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("cargo::rerun-if-changed=src/data.rs");
    println!("cargo::rerun-if-changed=build/highlight.rs");

    let source =
        std::fs::read_to_string("src/data.rs").map_err(|e| format!("read src/data.rs: {e}"))?;
    let snippets = highlight::snippets(&source).map_err(|e| format!("parse src/data.rs: {e}"))?;

    let mut out = String::from(
        "/// Each snippet's trimmed code and its highlighted HTML.\npub(crate) static HIGHLIGHTED: &[(&str, &str)] = &[\n",
    );
    for snippet in snippets {
        let code = snippet.code.trim();
        let html = highlight::highlight(&snippet.lang, code)
            .map_err(|e| format!("highlight a {} snippet: {e}", snippet.lang))?;
        writeln!(out, "    ({code:?}, {html:?}),")?;
    }
    out.push_str("];\n");

    let dir =
        std::env::var("OUT_DIR").map_err(|e| format!("OUT_DIR is set for build scripts: {e}"))?;
    std::fs::write(std::path::Path::new(&dir).join("highlighted.rs"), out)
        .map_err(|e| format!("write highlighted.rs: {e}"))?;
    Ok(())
}
