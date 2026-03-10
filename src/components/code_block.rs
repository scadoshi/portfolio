use dioxus::prelude::*;

static BLOCK_ID: GlobalSignal<u32> = Signal::global(|| 0);

#[component]
pub fn CodeBlock(title: String, code: String, description: String) -> Element {
    let trimmed = code.trim().to_string();
    let id = use_hook(|| {
        let current = *BLOCK_ID.peek();
        *BLOCK_ID.write() = current + 1;
        format!("code-block-{current}")
    });

    let id_clone = id.clone();
    use_effect(move || {
        let id = id_clone.clone();
        spawn(async move {
            gloo_timers::future::TimeoutFuture::new(50).await;
            let js = format!(
                r#"
                var el = document.querySelector('#{} code');
                if (el && typeof hljs !== 'undefined') {{
                    el.removeAttribute('data-highlighted');
                    hljs.highlightElement(el);
                }}
                "#,
                id
            );
            let _ = document::eval(&js).await;
        });
    });

    rsx! {
        div { class: "code-block",
            div { class: "code-header", "{title}" }
            pre { class: "code-content", id: "{id}",
                code { class: "language-rust", "{trimmed}" }
            }
            if !description.is_empty() {
                p { class: "code-description", "{description}" }
            }
        }
    }
}
