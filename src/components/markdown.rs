use comrak::{markdown_to_html, ComrakOptions};
use leptos::*;

#[component]
pub fn Markdown(cx: Scope, content: String) -> impl IntoView {
    let content = markdown_to_html(&content, &ComrakOptions::default());
    view! {cx, <div class="prose dark:prose-invert" inner_html=content></div>}
}
