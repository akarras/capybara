use capybara_lemmy_client::{
    post::{GetPosts, PostView},
    CapyClient,
};
use leptos::*;

use crate::{
    app::{CurrentUser, ErrorView},
    community::CommunityKey,
    components::{
        feed::{post_preview::PostPreview, virtual_scroll::InfinitePage},
        post_view_controls::PostViewControls,
        sorting_components::{SortMenu, TypeMenu},
    },
};

#[component]
pub fn Posts(
    cx: Scope,
    #[prop(into, optional)] community: Option<Signal<Option<CommunityKey>>>,
) -> impl IntoView {
    let (sort, set_sort) = create_signal(cx, None);
    let (type_, set_type) = create_signal(cx, None);
    let user = use_context::<CurrentUser>(cx).unwrap();
    let posts = create_local_resource(
        cx,
        move || (sort(), type_(), user.0(), community.and_then(|c| c())),
        move |(sort, type_, _, community)| async move {
            let client = use_context::<CapyClient>(cx).expect("need client");
            let community_name = community.as_ref().and_then(|c| c.name());
            let community_id = community.as_ref().and_then(|c| c.id());
            client
                .execute(GetPosts {
                    sort,
                    type_,
                    community_name,
                    community_id,
                    ..Default::default()
                })
                .await
                .map_err(leptos::error::Error::from)
        },
    );
    view! { cx,
        <div class="flex flex-row sticky top-10 h-10 bg-neutral-700 w-fit z-50">
                <SortMenu sort set_sort/>
                <TypeMenu type_ set_type/>
                <div class="w-5"></div>
                <PostViewControls />
        </div>
        <div class="flex flex-col">
            
            <Suspense fallback=move || {
                view! { cx, "Loading" }
            }>
                {move || {
                    posts
                        .with(
                            cx,
                            move |p| {
                                view! { cx,
                                    <ErrorView
                                        value=p.clone()
                                        ok=move |p| {
                                            let posts = p.posts;
                                            let sort = sort();
                                            let type_ = type_();
                                            let community_temp = community.and_then(|c| c.get());
                                            let community_id = community_temp.as_ref().and_then(|id| id.id());
                                            let community_name = community_temp.as_ref().and_then(|name| name.name());
                                            let posts = create_rw_signal(cx, posts);
                                            view! { cx,
                                                <InfinitePage
                                                    get_page=move |page| async move {
                                                        let client = use_context::<CapyClient>(cx).expect("need client");
                                                        let community = community.and_then(|c| c.get_untracked());
                                                        let community_id = community.as_ref().and_then(|id| id.id());
                                                        let community_name = community.as_ref().and_then(|name| name.name());
                                                        client
                                                            .execute(GetPosts {
                                                                page: Some(page as i64),
                                                                type_,
                                                                sort,
                                                                community_id,
                                                                community_name,
                                                                ..Default::default()
                                                            })
                                                            .await
                                                            .unwrap()
                                                            .posts
                                                    }
                                                    key=move |p: &PostView| p.post.id
                                                    view=move |cx, post| {
                                                        view! { cx, <PostPreview post/> }
                                                    }
                                                    data=posts
                                                    cache_key=(("posts", sort, type_, community_id, community_name))
                                                />
                                            }
                                        }
                                    />
                                }
                            },
                        )
                }}
            </Suspense>
        </div>
    }
}
