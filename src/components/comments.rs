use std::collections::HashMap;

use capybara_lemmy_client::{
    comment::{Comment, CommentAggregates, CommentId, CommentView, GetComments},
    post::PostId,
    CapyClient,
};
use leptos::*;
use log::info;

use crate::components::{community::CommunityBadge, person::PersonView};

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
    view! { cx,
        <div class="border-l-red-300 border-l-2">
            <div class="flex flex-row">
                <div>{child_count}" comments"</div>
                <div>{upvotes}" u"</div>
                <div>{score}" s"</div>
                <div>{downvotes}" d"</div>
                <div on:click=move |_| { set_collapsed(!collapsed())}>{move || if collapsed() { "show" } else { "hide" }}</div>
            </div>
            <div class="flex flex-col" class:hidden=collapsed>
                <div class="flex flex-row">
                    <PersonView person=creator/>
                    <CommunityBadge community />
                </div>
                <div class="prose">{content}</div>
                <div class="flex flex-row">{saved.then(|| "saved")}" "{}</div>
                <div class="m-5">
                    {children.into_iter().map(|comment| {
                        view!{cx, <Comment comment/>}
                    }).collect::<Vec<_>>()}
                </div>
            </div>
        </div>
    }
}

#[component]
pub fn PostComments(cx: Scope, post_id: PostId) -> impl IntoView {
    let post_comments = create_resource(
        cx,
        move || {},
        move |_| async move {
            let client = use_context::<CapyClient>(cx).unwrap();
            let comments = client
                .execute(GetComments {
                    post_id: Some(post_id),
                    ..Default::default()
                })
                .await
                .unwrap();
            comments
        },
    );

    view! {cx,
    <Suspense fallback=move || view!{cx, "Loading"}>
        {move || post_comments.read(cx).map(|comments| {
            let comments = CommentWithChildren::from_comments(comments.comments);
            comments.into_iter().map(|comment| {
                view!{cx, <Comment comment=comment />}
            }).collect::<Vec<_>>()
        })}
    </Suspense>
    }
}
