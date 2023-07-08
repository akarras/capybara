use capybara_lemmy_client::{
    community::{CommunityAggregates, CommunityView, ListCommunities},
    post::SortType,
    CapyClient,
};
use leptos::*;

use crate::{
    app::{CurrentUser, ErrorView},
    components::{
        community::CommunityBadge,
        feed::virtual_scroll::InfinitePage,
        numbers::NumberVis,
        sorting_components::{SortMenu, TypeMenu},
        subscribe::SubscribeButton,
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
        id,
        community_id,
        subscribers,
        posts,
        comments,
        published,
        users_active_day,
        users_active_week,
        users_active_month,
        users_active_half_year,
        hot_rank,
    } = counts;
    let subscribed = create_rw_signal(cx, subscribed);
    view! { cx,
        <div class="flex flex-row p-5 border-4 border-neutral-700 bg-neutral-800 text-neutral-100 gap-5">
            <div class="flex flex-col">
                <CommunityBadge community subscribed />
                <div><NumberVis value=subscribers/>" subscribers"</div>
                <div><NumberVis value=posts/>" posts"</div>
                <div><NumberVis value=comments/> " comments"</div>
            </div>
            <div class="flex flex-col">
                <div><NumberVis value=users_active_day/>" daily active users"</div>
                <div><NumberVis value=users_active_week/>" weekly active users"</div>
                <div><NumberVis value=users_active_month/> " monthly active users"</div>
                <div><NumberVis value=users_active_half_year/> " half year active users"</div>
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
                    ..Default::default()
                })
                .await
                .map_err(leptos::error::Error::from)
        },
    );
    view! { cx,
        <div>
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
            <Suspense fallback=move || {
                view! { cx, "Loading" }
            }>
                {move || {
                    let communities = communities.read(cx);
                    communities
                        .map(|community| {
                            view! { cx,
                                <ErrorView
                                    value=community
                                    ok=move |communities| {
                                        view! { cx,
                                            <InfinitePage
                                                get_page=move |page| {
                                                    async move {
                                                        let client = use_context::<CapyClient>(cx).unwrap();
                                                        let show_nsfw = show_nsfw.get_untracked();
                                                        let type_ = type_.get_untracked();
                                                        let sort = sort.get_untracked();
                                                        client
                                                            .execute(ListCommunities {
                                                                show_nsfw,
                                                                type_,
                                                                sort,
                                                                page: Some(page as i64),
                                                                ..Default::default()
                                                            })
                                                            .await
                                                            .ok()
                                                            .map(|c| c.communities)
                                                            .unwrap_or_default()
                                                    }
                                                }
                                                initial_data=communities.communities
                                                key=|c| c.community.id
                                                view=|cx, community| {
                                                    view! { cx, <CommunityView community/> }
                                                }
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
