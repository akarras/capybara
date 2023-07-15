use capybara_lemmy_client::{
    community::{CommunityAggregates, CommunityView, ListCommunities},
    post::SortType,
    CapyClient,
};
use leptos::*;
use leptos_icons::{BsIcon, FaIcon, Icon};

use crate::{
    app::{CurrentUser, ErrorView},
    components::{
        community::CommunityBadge,
        feed::virtual_scroll::InfinitePage,
        numbers::NumberVis,
        sorting_components::{SortMenu, TypeMenu},
        subscribe::SubscribeButton,
        time::RelativeTime,
    },
};

#[component]
pub fn CommunityView(cx: Scope, community: CommunityView) -> impl IntoView {
    let CommunityView {
        community,
        subscribed,
        blocked,
        counts,
    } = community;
    let CommunityAggregates {
        community_id,
        subscribers,
        posts,
        comments,
        published,
        users_active_month,
        ..
    } = counts;
    let subscribed = create_rw_signal(cx, subscribed);
    view! { cx,
        <div class="flex flex-col p-5 border-2 border-neutral-700 bg-neutral-800 text-neutral-100 gap-2">
            <div class="flex flex-row gap-1 text-2xl">
                <CommunityBadge community subscribed />
                "created: "<RelativeTime time=published />
            </div>
            <div class="flex flex-row leading-none gap-1 p-1 text-xl">
                <Icon icon=MaybeSignal::Static(BsIcon::BsPeopleFill.into()) /><NumberVis value=subscribers/>
                <div><NumberVis value=posts/>" posts"</div>
                <Icon icon=MaybeSignal::Static(FaIcon::FaCommentsSolid.into()) /><NumberVis value=comments/>
                <div class="flex flex-row leading-none"><Icon icon=MaybeSignal::Static(BsIcon::BsPeopleFill.into()) /><NumberVis value=users_active_month/> " monthly active users"</div>
            </div>
            <SubscribeButton community_id subscribed />
        </div>
    }
}

#[component]
pub fn CommunityList(cx: Scope) -> impl IntoView {
    let (show_nsfw, set_show_nsfw) = create_signal(cx, None);
    let (sort, set_sort) = create_signal(cx, Some(SortType::TopAll));
    let (type_, set_type) = create_signal(cx, None);
    let current_user = use_context::<CurrentUser>(cx).unwrap();
    let communities = create_local_resource(
        cx,
        move || (show_nsfw(), type_(), sort(), current_user()),
        move |(show_nsfw, type_, sort, _)| async move {
            let client = use_context::<CapyClient>(cx).unwrap();
            client
                .execute(ListCommunities {
                    show_nsfw,
                    type_,
                    sort,
                    limit: Some(50),
                    ..Default::default()
                })
                .await
                .map_err(leptos::error::Error::from)
        },
    );
    view! { cx,
        <div>
            <div class="flex flex-row">
                <SortMenu sort set_sort />
                <TypeMenu type_ set_type />
                <button class="p-1 bg-neutral-800 hover:bg-neutral-600" on:click=move |_| {
                    set_show_nsfw(match show_nsfw.get_untracked() {
                        Some(true) => Some(false),
                        Some(false) => None,
                        None => Some(true)
                    })
                }>{move || match show_nsfw() {
                    Some(true) => "nsfw only",
                    Some(false) => "no nsfw",
                    None => "nsfw filter not set"
                }}</button>
            </div>
            <Suspense fallback=move || {
                view! { cx, "Loading" }
            }>
                {move || {
                    let communities = communities.read(cx);
                    let show_nsfw = show_nsfw();
                    let type_ = type_();
                    let sort = sort();
                    communities
                        .map(|community| {
                            view! { cx,
                                <ErrorView
                                    value=community
                                    ok=move |communities| {
                                        let data = create_rw_signal(cx, communities.communities);
                                        view! { cx,
                                            <InfinitePage
                                                get_page=move |page| {
                                                    async move {
                                                        let client = use_context::<CapyClient>(cx).unwrap();
                                                        client
                                                            .execute(ListCommunities {
                                                                show_nsfw,
                                                                type_,
                                                                sort,
                                                                page: Some(page as i64),
                                                                limit: Some(50),
                                                                ..Default::default()
                                                            })
                                                            .await
                                                            .ok()
                                                            .map(|c| c.communities)
                                                            .unwrap_or_default()
                                                    }
                                                }
                                                data
                                                key=|c| c.community.id
                                                view=|cx, community| {
                                                    view! { cx, <CommunityView community/> }
                                                }
                                                cache_key=("community_list", show_nsfw, type_, sort)
                                            />
                                        }
                                    }
                                />
                            }
                        })
                }}
            </Suspense>
        </div>
    }
}
