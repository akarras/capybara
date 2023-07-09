use crate::components::{comments::PostComments, feed::post_preview::PostPreview};
use capybara_lemmy_client::{
    post::{GetPost, PostId},
    CapyClient,
};
use leptos::*;
use leptos_router::{use_params_map, use_query_map};
use log::info;

#[component]
pub fn Post(cx: Scope) -> impl IntoView {
    let params = use_params_map(cx);
    let post_id = create_memo(cx, move |_| {
        params.with(|q| q.0.get("id").map(|i| i.parse::<i32>().ok()).flatten())
    });
    let resource = create_resource(
        cx,
        move || post_id(),
        move |id| async move {
            let client = use_context::<CapyClient>(cx).unwrap();
            let id = id?;
            let response = client
                .execute(GetPost {
                    id: Some(PostId(id)),
                    ..Default::default()
                })
                .await
                .unwrap();
            Some(response)
        },
    );
    create_effect(cx, move |_| {
        info!("{:?}", resource.read(cx));
    });
    view! { cx,
        <button
            class="bg-gray-800 text-white"
            on:click=move |_| {
                let _ = window().history().ok().unwrap().back();
            }
        >
            "BACK"
        </button>
        <Suspense fallback=move || {
            view! { cx, "Loading" }
        }>
            {move || {
                resource
                    .read(cx)
                    .map(|post_opt| {
                        post_opt
                            .map(|post| {
                                view! { cx, <PostPreview post=post.post_view/> }
                            })
                    })
            }}
        </Suspense>
        {move || post_id().map(|p| view!{cx, <PostComments post_id=PostId(p) />})}
    }
}
