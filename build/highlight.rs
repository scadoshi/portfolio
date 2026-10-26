//! Syntax highlighting for the project pages' code snippets, run at build time.
//!
//! `build.rs` reads every `Snippet { lang, code, .. }` literal out of
//! `src/data.rs` and highlights it with lumis, which parses the code with
//! tree-sitter and tags it with Neovim's highlight captures, the way an editor
//! does. The output is HTML spans with `l-*` classes (`l-function-call`,
//! `l-variable-parameter`, ...) that `assets/main.css` maps onto the theme's
//! `--syntax-*` slots. The tests in `tests/highlight.rs` include this file too.

use lumis::HtmlLinkedBuilder;
use lumis::languages::Language;
use syn::visit::Visit;

/// One `Snippet` literal from the data file.
pub(crate) struct Snippet {
    pub lang: String,
    pub code: String,
}

/// Every `Snippet { .. }` struct literal in `source` that has a string `lang`
/// and a string `code`, in file order.
pub(crate) fn snippets(source: &str) -> syn::Result<Vec<Snippet>> {
    struct Finder(Vec<Snippet>);

    impl<'ast> Visit<'ast> for Finder {
        fn visit_expr_struct(&mut self, node: &'ast syn::ExprStruct) {
            if node.path.is_ident("Snippet") {
                let field = |name: &str| {
                    node.fields.iter().find_map(|f| match (&f.member, &f.expr) {
                        (
                            syn::Member::Named(ident),
                            syn::Expr::Lit(syn::ExprLit {
                                lit: syn::Lit::Str(s),
                                ..
                            }),
                        ) if ident == name => Some(s.value()),
                        _ => None,
                    })
                };
                if let (Some(lang), Some(code)) = (field("lang"), field("code")) {
                    self.0.push(Snippet { lang, code });
                }
            }
            syn::visit::visit_expr_struct(self, node);
        }
    }

    let file = syn::parse_file(source)?;
    let mut finder = Finder(Vec::new());
    finder.visit_file(&file);
    Ok(finder.0)
}

/// The tree-sitter grammar for a snippet's `lang`.
pub(crate) fn language(lang: &str) -> Option<Language> {
    match lang {
        "rust" => Some(Language::Rust),
        "csharp" => Some(Language::CSharp),
        _ => None,
    }
}

/// The highlighted HTML that goes inside the snippet's `<code>` element.
pub(crate) fn highlight(lang: &str, code: &str) -> Result<String, String> {
    let language = language(lang).ok_or_else(|| format!("no grammar for lang {lang:?}"))?;
    let formatter = HtmlLinkedBuilder::new()
        .language(language)
        .build()
        .map_err(|e| e.to_string())?;
    let html = lumis::highlight(code, formatter);
    Ok(macros_as_macros(&inner_lines(&html)))
}

/// The lines inside lumis's `<pre><code>` wrapper, each unwrapped from its
/// `l-line` span. The block's `white-space: pre` breaks the lines.
fn inner_lines(html: &str) -> String {
    let after_open = html
        .split_once("<code")
        .and_then(|(_, rest)| rest.split_once('>'))
        .map_or(html, |(_, inner)| inner);
    let inner = after_open
        .rsplit_once("</code>")
        .map_or(after_open, |(inner, _)| inner);
    inner
        .split('\n')
        .map(|line| match line.strip_prefix("<span class=\"l-line\"") {
            Some(rest) => {
                let body = rest.split_once('>').map_or(rest, |(_, body)| body);
                body.strip_suffix("</span>").unwrap_or(body)
            }
            None => line,
        })
        .collect::<Vec<_>>()
        .join("\n")
}

/// Gives every Rust macro name the macro class. The tree-sitter queries tag
/// `panic!`, `eprintln!` and their kin as exceptions or debug keywords, and
/// rust-analyzer's semantic tokens, which an editor layers on top, color them
/// as the macros they are.
fn macros_as_macros(html: &str) -> String {
    let mut out = String::with_capacity(html.len());
    let mut rest = html;
    while let Some((before, span)) = rest
        .find("<span class=\"")
        .and_then(|i| rest.split_at_checked(i))
    {
        out.push_str(before);
        // The opening tag up to its `>`; a tag that never closes is all that is left.
        let (open, after_tag) = match span.split_once('>') {
            Some((open, after)) => (Some(open), after),
            None => (None, ""),
        };
        let (text, tail) = after_tag
            .find('<')
            .and_then(|j| after_tag.split_at_checked(j))
            .unwrap_or((after_tag, ""));
        let is_macro = text.strip_suffix('!').is_some_and(|name| {
            !name.is_empty() && name.chars().all(|c| c.is_alphanumeric() || c == '_')
        });
        if is_macro {
            out.push_str("<span class=\"l-function-macro\">");
        } else if let Some(open) = open {
            out.push_str(open);
            out.push('>');
        } else {
            out.push_str(span);
        }
        out.push_str(text);
        rest = tail;
    }
    out.push_str(rest);
    out
}
