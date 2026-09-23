use dioxus::prelude::*;

mod components;
mod data;
mod pages;
mod theme_store;

use pages::{
    contribute::Contribute,
    detail::{ProjectDetail, SideQuestDetail},
    home::Home,
    not_found::NotFound,
    side_quests::SideQuests,
};
use zwipe_components::{COMPONENTS_CSS, THEMES_CSS, ThemeConfig};

const MAIN_CSS: Asset = asset!("/assets/main.css");
const REVEAL_JS: Asset = asset!("/assets/reveal.js");
const FAVICON_ICO: Asset = asset!("/assets/favicon/favicon.ico");
const FAVICON_16: Asset = asset!("/assets/favicon/favicon-16x16.png");
const FAVICON_32: Asset = asset!("/assets/favicon/favicon-32x32.png");
const APPLE_TOUCH_ICON: Asset = asset!("/assets/favicon/apple-touch-icon.png");

#[derive(Debug, Clone, Routable, PartialEq)]
#[rustfmt::skip]
pub enum Route {
    #[layout(NavbarLayout)]
        #[route("/")]
        Home {},
        #[route("/projects/:slug")]
        ProjectDetail { slug: String },
        #[route("/side-quests")]
        SideQuests {},
        #[route("/side-quests/:slug")]
        SideQuestDetail { slug: String },
        #[route("/contribute")]
        Contribute {},
        // GitHub Pages serves 404.html (a copy of the app shell, made in
        // deploy.yml) for unknown paths, and the router lands here.
        #[route("/:..segments")]
        NotFound { segments: Vec<String> },
}

fn main() {
    dioxus::LaunchBuilder::new()
        .with_cfg(server_only! {
            dioxus::server::ServeConfig::builder()
                .incremental(
                    dioxus::server::IncrementalRendererConfig::new()
                        // Both of these are fatal and unrecoverable: without
                        // the exe's directory there is nowhere to write the
                        // prerendered pages, so panicking with a real message
                        // beats a bare unwrap in a build log.
                        .static_dir(
                            std::env::current_exe()
                                .expect("current exe path unavailable")
                                .parent()
                                .expect("current exe has no parent directory")
                                .join("public"),
                        )
                        .clear_cache(false),
                )
        })
        .launch(App);
}

// Nothing here awaits, but `#[server]` only accepts an async fn: it wraps the
// body in a future and calls it across the network boundary. Removing `async`
// to satisfy clippy stops it compiling.
#[allow(clippy::unused_async)]
#[server(endpoint = "static_routes")]
async fn static_routes() -> ServerFnResult<Vec<String>> {
    let mut routes: Vec<String> = Route::static_routes()
        .iter()
        .map(ToString::to_string)
        .collect();
    for p in data::featured_projects() {
        routes.push(format!("/projects/{}", p.slug));
    }
    for p in data::side_quests() {
        routes.push(format!("/side-quests/{}", p.slug));
    }
    // Prerendered through the catch-all so deploy.yml can ship the result as
    // 404.html. Copying index.html there instead gives every unknown URL the
    // Home page's title, canonical and JSON-LD, with no noindex.
    routes.push("/404".to_string());
    Ok(routes)
}

#[component]
fn App() -> Element {
    // Start at the default so the first client render matches the server's.
    // Seeding from localStorage here would desync hydration: it keeps the
    // server DOM (picker label, wrapper theme class) and won't reconcile the
    // mismatch. The stored theme is adopted after mount instead.
    let mut theme = use_signal(ThemeConfig::default);
    use_context_provider(|| theme);
    let mut loaded = use_signal(|| false);

    // Post-mount state change, so the picker label and theme wrapper re-render.
    use_effect(move || {
        if let Some(stored) = theme_store::load() {
            theme.set(stored);
        }
        loaded.set(true);
    });

    // Persist every theme change so the next visit opens in it. The `loaded`
    // guard keeps the pre-load default render from clobbering the stored theme.
    use_effect(move || {
        let cfg = theme.read().clone();
        if loaded() {
            theme_store::save(&cfg);
        }
    });

    rsx! {
        document::Meta { name: "viewport", content: "width=device-width, initial-scale=1, viewport-fit=cover" }
        // Tells Dark Reader to leave the site alone (same lock zite carries):
        // theming is first-class here, and Dark Reader's dynamic mode mangles
        // the color-mix()/var() palette.
        document::Meta { name: "darkreader-lock" }
        // Fonts are self-hosted in public/fonts (see the @font-face block in
        // main.css); preloading the two latin weights starts those fetches
        // before CSS parsing discovers them, closing the fallback-font flash.
        document::Link { rel: "preload", href: "/fonts/jetbrains-mono-latin-400-normal.woff2", r#as: "font", r#type: "font/woff2", crossorigin: "anonymous" }
        document::Link { rel: "preload", href: "/fonts/jetbrains-mono-latin-700-normal.woff2", r#as: "font", r#type: "font/woff2", crossorigin: "anonymous" }
        document::Link { rel: "icon", href: FAVICON_ICO }
        document::Link { rel: "icon", r#type: "image/png", sizes: "32x32", href: FAVICON_32 }
        document::Link { rel: "icon", r#type: "image/png", sizes: "16x16", href: FAVICON_16 }
        document::Link { rel: "apple-touch-icon", sizes: "180x180", href: APPLE_TOUCH_ICON }
        // Shared CSS inlined from zwipe-components (a git dep can't be reached
        // by an asset pipeline). Order matters: themes -> components -> site,
        // so site rules can override component rules.
        document::Style { {THEMES_CSS} }
        document::Style { {COMPONENTS_CSS} }
        document::Stylesheet { href: MAIN_CSS }
        // Deferred so the CDN fetch doesn't block first paint; CodeBlock only
        // calls hljs from a post-hydration effect (with a typeof guard), long
        // after deferred scripts have executed.
        document::Script { defer: true, src: "https://cdnjs.cloudflare.com/ajax/libs/highlight.js/11.9.0/highlight.min.js" }
        document::Script { defer: true, src: "https://cdnjs.cloudflare.com/ajax/libs/highlight.js/11.9.0/languages/rust.min.js" }
        // Sharpmas's snippets are C#. Without this grammar they fell back to
        // Rust's, which colors five keywords the two languages share and
        // leaves public, sealed, record, interface, var and switch plain.
        document::Script { defer: true, src: "https://cdnjs.cloudflare.com/ajax/libs/highlight.js/11.9.0/languages/csharp.min.js" }
        // Scroll reveal for panels below the fold; deferred for the same
        // first-paint reason, and everything it does is progressive.
        document::Script { defer: true, src: REVEAL_JS }
        Router::<Route> {}
    }
}

#[component]
fn NavbarLayout() -> Element {
    let theme = use_context::<Signal<ThemeConfig>>();
    // Core's css_class() is just "theme-{name}-{mode}"; the wrapper class
    // carries this site's fixed-grid background layer.
    let css_class = theme.read().css_class();
    rsx! {
        div { class: "theme-wrapper {css_class}",
            components::navbar::Navbar {}
            main { class: "content",
                Outlet::<Route> {}
            }
            components::footer::Footer {}
        }
    }
}
