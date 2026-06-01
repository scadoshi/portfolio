use dioxus::prelude::*;

#[component]
pub fn PageMeta(title: String, description: String, path: String) -> Element {
    let canonical = format!("https://scottyfermo.com{path}");
    let full_title = if title == "Scotty Fermo" {
        "Scotty Fermo".to_string()
    } else {
        format!("{title} | Scotty Fermo")
    };

    rsx! {
        document::Title { "{full_title}" }
        document::Meta { name: "description", content: "{description}" }
        document::Link { rel: "canonical", href: "{canonical}" }

        document::Meta { property: "og:type", content: "website" }
        document::Meta { property: "og:site_name", content: "Scotty Fermo" }
        document::Meta { property: "og:title", content: "{full_title}" }
        document::Meta { property: "og:description", content: "{description}" }
        document::Meta { property: "og:url", content: "{canonical}" }

        document::Meta { name: "twitter:card", content: "summary" }
        document::Meta { name: "twitter:title", content: "{full_title}" }
        document::Meta { name: "twitter:description", content: "{description}" }
    }
}
