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
        // Catch-all 404: GitHub Pages serves 404.html (a copy of the app
        // shell, made in deploy.yml) for unknown paths, and the router lands
        // here. Excluded from static_routes automatically (dynamic segments).
        #[route("/:..segments")]
        NotFound { segments: Vec<String> },
}

fn main() {
    dioxus::LaunchBuilder::new()
        .with_cfg(server_only! {
            dioxus::server::ServeConfig::builder()
                .incremental(
                    dioxus::server::IncrementalRendererConfig::new()
                        .static_dir(
                            std::env::current_exe()
                                .unwrap()
                                .parent()
                                .unwrap()
                                .join("public"),
                        )
                        // Clear cached renders on startup: the incremental
                        // renderer caches every visited path (404s included)
                        // as HTML on disk, and stale copies from before a
                        // redesign would otherwise keep being served in dev.
                        // Only per-route .html files are cleared; assets and
                        // the shell are untouched, and SSG re-renders every
                        // route after the clear.
                        .clear_cache(true),
                )
        })
        .launch(App);
}

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
    Ok(routes)
}

#[component]
fn App() -> Element {
    // Start at the default so the client's first render matches the server's
    // (localStorage is client-only). Seeding from storage here would desync SSR
    // and hydration: hydration keeps the server DOM (e.g. the picker's label and
    // the wrapper theme class) and won't reconcile the mismatch. Instead adopt
    // the stored theme just after mount (below).
    let mut theme = use_signal(ThemeConfig::default);
    use_context_provider(|| theme);
    let mut loaded = use_signal(|| false);

    // After hydration, adopt the last-used theme from localStorage. Being a
    // post-mount state change, this re-renders the picker label and the theme
    // wrapper too, not just future interactions.
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
