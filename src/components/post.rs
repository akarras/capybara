use capybara_lemmy_client::{
    post::{GetPost, PostId},
    CapyClient,
};
use leptos::*;
use leptos_router::{use_query_map};

use crate::components::feed::post_preview::PostPreview;

#[component]
pub fn Post(cx: Scope) -> impl IntoView {
    let query = use_query_map(cx);
    let post_id = create_memo(cx, move |_| {
        query.with(|q| q.0.get("id").map(|i| i.parse::<i32>().ok()).flatten())
    });
    let resource = create_resource(
        cx,
        move || post_id(),
        move |id| async move {
            let client = use_context::<CapyClient>(cx).unwrap();
            let id = id?;
            let response = client
                .get_post(GetPost {
                    id: Some(PostId(id)),
                    ..Default::default()
                })
                .await
                .ok()?;
            Some(response)
        },
    );

    view! {cx,
    <Suspense fallback=move || view!{cx, "Loading"}>
        {move || {resource.read(cx).map(|post_opt| {
            post_opt.map(|post| {
                view!{cx, <PostPreview post=post.post_view/>}
            })
        })}}
    </Suspense>}
}
