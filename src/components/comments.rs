use std::collections::HashMap;

use capybara_lemmy_client::{
    comment::{
        Comment, CommentAggregates, CommentSortType, CommentView, CreateComment, CreateCommentLike,
        GetComments, SaveComment,
    },
    post::PostId,
    CapyClient,
};
use leptos::*;
use leptos_icons::{BiIcon, BsIcon, Icon};
use log::info;
use serde::{Deserialize, Serialize};

use crate::{
    app::CurrentUser,
    components::{
        feed::virtual_scroll::InfinitePage, markdown::Markdown, person::PersonView,
        reply_box::ReplyBox, save_button::SaveButton, sorting_components::CommentSortMenu,
        time::RelativeTime, voter::Voter,
    },
};

#[derive(Serialize, Deserialize, Clone)]
pub struct CommentWithChildren(pub CommentView, pub Vec<CommentWithChildren>);

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
    // let subscribed = create_rw_signal(cx, subscribed);
    let my_vote = create_rw_signal(cx, my_vote);
    let user = use_context::<CurrentUser>(cx).unwrap();
    let (saved, set_saved) = create_signal(cx, saved);
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
    create_effect(cx, move |prev| {
        let saved = saved();
        let user = user();
        if let Some((Some(prev_user), prev)) = prev {
            if Some(prev_user) == user && saved != prev {
                spawn_local(async move {
                    let client = use_context::<CapyClient>(cx).unwrap();
                    let like = SaveComment {
                        comment_id,
                        save: saved,
                        ..Default::default()
                    };
                    let _ = client.execute(like).await;
                });
            }
        }
        (user, saved)
    });
    let children = create_rw_signal(cx, children);
    let (reply, set_reply) = create_signal(cx, false);
    view! { cx,
        <div class="flex flex-row border-neutral-700 hover:border-neutral-600 border-solid border-t-2">
            <button
                class="p-1 bg-red-300 hover:bg-red-600 border-1 border-gray-200"
                on:click=move |_| { set_collapsed(!collapsed()) }
            ></button>
            <Voter my_vote upvotes downvotes score/>
            <div class="flex flex-col grow transition" class:hidden=collapsed>
                <div class="flex flex-row">
                    <div class="flex flex-row text-gray-500">
                        <RelativeTime time=published/>
                        {updated
                            .map(|u| {
                                view! { cx,
                                    "(updated: "
                                    <RelativeTime time=u/>
                                    ")"
                                }
                            })}
                    </div>
                </div>
                <PersonView person=creator/>
                <div class="p-2">
                    <Markdown content/>
                </div>
                <div class="flex flex-row gap-2">
                    <div
                        class=move || {
                            if reply() {
                                "flex flex-row bold text-yellow-500 hover:text-yellow-400 bg-gray-500 rounded leading-none"
                            } else {
                                "flex flex-row bold text-gray-500 hover:text-gray-400 leading-none"
                            }
                        }
                        on:click=move |_| {
                            set_reply(!reply());
                        }
                    >
                        <Icon icon=MaybeSignal::Static(BsIcon::BsReplyFill.into())/>
                        " reply"
                    </div>
                    <SaveButton saved set_saved/>
                </div>
                <ReplyBox post_id parent_id=Some(comment_id) reply_open=reply set_reply_open=set_reply children />
                <div class="">
                    {move || children()
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
    let (sort, set_sort) = create_signal(cx, Some(CommentSortType::Hot));
    let limit = Some(50);
    let post_comments = create_resource(
        cx,
        move || sort(),
        move |sort| async move {
            let client = use_context::<CapyClient>(cx).unwrap();
            let comments = client
                .execute(GetComments {
                    post_id: Some(post_id),
                    sort,
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
        <div class="flex flex-row">
            <CommentSortMenu sort set_sort/>
        </div>
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
                                                sort: sort.get_untracked(),
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
                                cache_key=("comment_view", post_id, sort())
                            />
                        }
                    })
            }}
        </Suspense>
    }
}
