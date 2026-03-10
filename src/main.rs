use dioxus::prelude::*;

mod components;
mod data;
mod pages;
mod theme;

use pages::home::Home;
use pages::project_detail::ProjectDetail;
use pages::side_quests::SideQuests;
use theme::ThemeConfig;

const MAIN_CSS: Asset = asset!("/assets/main.css");

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
}

fn main() {
    dioxus::launch(App);
}

#[component]
fn App() -> Element {
    let theme = use_signal(|| ThemeConfig::default());
    use_context_provider(|| theme);
    rsx! {
        document::Stylesheet { href: MAIN_CSS }
        Router::<Route> {}
    }
}

#[component]
fn NavbarLayout() -> Element {
    let theme = use_context::<Signal<ThemeConfig>>();
    let css_class = theme.read().css_class();
    rsx! {
        div { class: "{css_class}",
            components::navbar::Navbar {}
            main { class: "content",
                Outlet::<Route> {}
            }
            components::footer::Footer {}
        }
    }
}
