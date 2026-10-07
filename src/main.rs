use dioxus::prelude::*;

mod components;
mod data;
mod pages;
mod stats;

use pages::{
    contribute::Contribute,
    detail::ProjectDetail,
    home::Home,
    moved::{OldSideQuest, OldSideQuests},
    not_found::NotFound,
    projects::{Projects, SectionPage},
};
use zwipe_components::{
    COMPONENTS_CSS, NAV_GLIDE_JS, REVEAL_JS, SCROLL_FADE_JS, SITE_CSS, THEMES_CSS, ThemeConfig,
    use_persisted_theme,
};

const MAIN_CSS: Asset = asset!("/assets/main.css");
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
        #[route("/projects")]
        Projects {},
        // Before the project route: only the four section names parse, so
        // every other slug falls through to ProjectDetail.
        #[route("/projects/:section")]
        SectionPage { section: data::Section },
        #[route("/projects/:slug")]
        ProjectDetail { slug: String },
        #[route("/contribute")]
        Contribute {},
        // Side quests were folded into the sections. These addresses only
        // ever render the redirect to where each page went (data::MOVED).
        #[route("/side-quests")]
        OldSideQuests {},
        #[route("/side-quests/:slug")]
        OldSideQuest { slug: String },
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
    // A moved static route (the old side-quest index) arrives with the MOVED
    // entries below, so it is skipped here rather than rendered twice.
    let mut routes: Vec<String> = Route::static_routes()
        .iter()
        .map(ToString::to_string)
        .filter(|path| data::moved_to(path).is_none())
        .collect();
    for section in data::Section::ALL {
        routes.push(format!("/projects/{section}"));
    }
    for p in data::all_projects() {
        routes.push(format!("/projects/{}", p.slug));
    }
    for moved in data::MOVED {
        routes.push(moved.from.to_string());
    }
    // Prerendered through the catch-all so deploy.yml can ship the result as
    // 404.html. Copying index.html there instead gives every unknown URL the
    // Home page's title, canonical and JSON-LD, with no noindex.
    routes.push("/404".to_string());
    Ok(routes)
}

#[component]
fn App() -> Element {
    // Starts at the default so hydration matches the prerendered page, adopts
    // the theme stored in localStorage just after mount and saves every pick.
    // The shell's script already put the wrapper on the stored theme, so the
    // adoption changes nothing visible.
    let theme = use_persisted_theme("zwipe.theme");
    use_context_provider(|| theme);

    // `hydrated` on the document releases the hero's entrance, which the
    // stylesheet holds until the app can run it.
    use_effect(move || {
        spawn(async {
            let _ = document::eval("document.documentElement.classList.add('hydrated');").await;
        });
    });

    // Live numbers from heron, asked for once after mount. Empty on the first
    // render so the client matches the prerendered page, then filled in place.
    let mut live: stats::Live = use_signal(|| None);
    use_context_provider(|| live);
    // The hero's entrance runs again whenever this moves; the nav S bumps it.
    let replay = zwipe_components::Replay(use_signal(|| 0u32));
    use_context_provider(|| replay);
    use_effect(move || {
        spawn(async move {
            if let Some(snapshot) = stats::fetch_live().await {
                live.set(Some(snapshot));
            }
        });
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
        // by an asset pipeline). Themes, then components, then the shared site
        // sheet. The prerendered head puts these inline blocks after the linked
        // main.css, so a main.css override of a shared selector needs higher
        // specificity, not just a later position.
        document::Style { {THEMES_CSS} }
        document::Style { {COMPONENTS_CSS} }
        document::Style { {SITE_CSS} }
        document::Stylesheet { href: MAIN_CSS }
        // highlight.js 11.9.0, vendored under public/ so the site makes no
        // third-party request. Deferred so it doesn't block first paint;
        // CodeBlock only calls hljs from a post-hydration effect (with a
        // typeof guard), long after deferred scripts have executed.
        document::Script { defer: true, src: "/vendor/highlight.min.js" }
        document::Script { defer: true, src: "/vendor/rust.min.js" }
        // Sharpmas's snippets are C#. Without this grammar they fell back to
        // Rust's, which colors five keywords the two languages share and
        // leaves public, sealed, record, interface, var and switch plain.
        document::Script { defer: true, src: "/vendor/csharp.min.js" }
        // Scroll reveal for everything marked `data-reveal` below the fold.
        // Inlined from zwipe-components; it waits for the document to parse,
        // as a deferred script would, and everything it does is progressive.
        document::Script { {REVEAL_JS} }
        // Nav items pushed by a wider theme label slide over instead of jumping.
        document::Script { {NAV_GLIDE_JS} }
        // Edge fades on the sideways chart scrollers where CSS can't drive
        // them (Firefox has no scroll timelines).
        document::Script { {SCROLL_FADE_JS} }
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
