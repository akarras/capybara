use std::collections::HashMap;

use capybara_lemmy_client::{
    comment::{
        Comment, CommentAggregates, CommentSortType, CommentView, CreateCommentLike, GetComments,
    },
    post::PostId,
    CapyClient,
};
use leptos::*;
use log::info;
use serde::{Deserialize, Serialize};

use crate::{
    app::CurrentUser,
    components::{
        community::CommunityBadge,
        feed::virtual_scroll::{InfinitePage, VirtualScroller},
        markdown::Markdown,
        person::PersonView,
        sorting_components::SortMenu,
        time::RelativeTime,
        voter::Voter,
    },
};

#[derive(Serialize, Deserialize, Clone)]
struct CommentWithChildren(CommentView, Vec<CommentWithChildren>);

fn get_children(
    comment: CommentView,
    comments: &mut HashMap<i32, Vec<CommentView>>,
) -> CommentWithChildren {
    let id = comment.comment.id.0;
    CommentWithChildren(
        comment,
        comments
            .remove(&id)
            .map(|children| {
                children
                    .into_iter()
                    .map(move |comment| get_children(comment, comments))
                    .collect()
            })
            .unwrap_or_default(),
    )
}

impl CommentWithChildren {
    fn from_comments(comments: Vec<CommentView>) -> Vec<CommentWithChildren> {
        let mut value: HashMap<i32, Vec<CommentView>> = comments
            .into_iter()
            .map(|c| {
                let second_to_last = c
                    .comment
                    .path
                    .split(".")
                    .collect::<Vec<_>>()
                    .into_iter()
                    .rev()
                    .nth(1)
                    .map(|p| p.parse::<i32>().unwrap())
                    .unwrap();
                (second_to_last, c)
            })
            .fold(HashMap::new(), |mut map, (parent, value)| {
                map.entry(parent).or_default().push(value);
                map
            });
        value
            .remove(&0)
            .map(|root_comments| {
                root_comments
                    .into_iter()
                    .map(move |comment| get_children(comment, &mut value))
                    .collect()
            })
            .unwrap_or_default()
    }
}

#[component]
fn Comment(cx: Scope, comment: CommentWithChildren) -> impl IntoView {
    let CommentWithChildren(comment, children) = comment;
    let CommentView {
        comment,
        creator,
        post,
        community,
        counts,
        creator_banned_from_community,
        subscribed,
        saved,
        creator_blocked,
        my_vote,
    } = comment;
    let Comment {
        id,
        creator_id,
        post_id,
        content,
        removed,
        published,
        updated,
        deleted,
        ap_id,
        local,
        path: _,
        distinguished,
        language_id,
    } = comment;
    let (collapsed, set_collapsed) = create_signal(cx, false);
    let CommentAggregates {
        id,
        comment_id,
        score,
        upvotes,
        downvotes,
        published,
        child_count,
        hot_rank,
    } = counts;
    let subscribed = create_rw_signal(cx, subscribed);
    let my_vote = create_rw_signal(cx, my_vote);
    let user = use_context::<CurrentUser>(cx).unwrap();
    create_effect(cx, move |prev| {
        let vote = my_vote();
        let user = user();
        if let Some((Some(prev_user), prev)) = prev {
            if Some(prev_user) == user && vote != prev {
                spawn_local(async move {
                    let client = use_context::<CapyClient>(cx).unwrap();
                    let like = CreateCommentLike {
                        comment_id,
                        score: vote.unwrap_or_default(),
                        ..Default::default()
                    };
                    // TODO: Actually update the comment somehow?
                    let _ = client.execute(like).await;
                    info!("liked comment");
                });
            }
        }
        (user, vote)
    });
    view! { cx,
        <div class="flex flex-row border-neutral-700 hover:border-neutral-600 border-solid border-t-2">
            <button
                class="p-1 bg-red-300 hover:bg-red-600 border-1 border-gray-200"
                on:click=move |_| { set_collapsed(!collapsed()) }
            ></button>
            <Voter my_vote upvotes downvotes score/>
            <div class="flex flex-col transition" class:hidden=collapsed>
                <div class="p-4">
                    <div class="flex flex-row">
                        <PersonView person=creator/>
                        <CommunityBadge community subscribed/>
                    </div>
                    <Markdown content/>
                    <div class="flex flex-row">
                        {saved.then(|| "saved")} " " {} " " <RelativeTime time=published/> {updated
                            .map(|u| {
                                view! { cx, <RelativeTime time=u/> }
                            })}
                    </div>
                </div>
                <div class="">
                    {children
                        .into_iter()
                        .map(|comment| {
                            view! { cx, <Comment comment/> }
                        })
                        .collect::<Vec<_>>()}
                </div>
            </div>
        </div>
    }
}

#[component]
pub fn PostComments(cx: Scope, post_id: PostId) -> impl IntoView {
    let (sort, set_sort) = create_signal(cx, CommentSortType::Hot);
    let limit = Some(50);
    let post_comments = create_resource(
        cx,
        move || sort(),
        move |sort| async move {
            let client = use_context::<CapyClient>(cx).unwrap();
            let comments = client
                .execute(GetComments {
                    post_id: Some(post_id),
                    sort: Some(sort),
                    limit,
                    page: None,
                    ..Default::default()
                })
                .await
                .unwrap();
            comments
        },
    );

    view! { cx,
        <div class="flex flex-row"><</div>
        <Suspense fallback=move || {
            view! { cx, "Loading" }
        }>
            {move || {
                post_comments
                    .read(cx)
                    .map(|comments| {
                        let comments = CommentWithChildren::from_comments(comments.comments);
                        view! { cx,
                            <InfinitePage
                                view=move |cx, comment| {
                                    view! { cx, <Comment comment=comment/> }
                                }
                                get_page=move |p| {
                                    async move {
                                        let client = use_context::<CapyClient>(cx).unwrap();
                                        let comments = client
                                            .execute(GetComments {
                                                post_id: Some(post_id),
                                                sort: Some(sort.get_untracked()),
                                                limit: None,
                                                page: Some(p as i64),
                                                ..Default::default()
                                            })
                                            .await
                                            .unwrap();
                                        CommentWithChildren::from_comments(comments.comments)
                                    }
                                }
                                initial_data=comments
                                key=|c| c.0.comment.id
                                cache_key=("comment_view", post_id)
                            />
                        }
                    })
            }}
        </Suspense>
    }
}
