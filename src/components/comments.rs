use std::collections::HashMap;

use capybara_lemmy_client::{
    comment::{Comment, CommentAggregates, CommentSortType, CommentView, GetComments},
    post::PostId,
    CapyClient,
};
use leptos::*;

use crate::components::{
    community::CommunityBadge, markdown::Markdown, person::PersonView, time::RelativeTime,
};

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
        <div class="border-l-red-300 border-l-2 bg-gray-900 p-4">
            <div class="flex flex-row gap-2">
                <div
                    class="p-1 rounded bg-gray-200 hover:bg-gray-600 broder-1 border-gray-200"
                    on:click=move |_| { set_collapsed(!collapsed()) }
                >
                    {move || if collapsed() { "+" } else { "-" }}
                </div>
                {(child_count != 0).then(|| view!{cx, <div>{child_count} " comments"</div>})}
                <div>{score} " score "</div>
                <div>{upvotes} "⬆️"</div>
                <div>{downvotes} "⬇️"</div>
            </div>
            <div class="flex flex-col transition" class:hidden=collapsed>
                <div class="flex flex-row">
                    <PersonView person=creator/>
                    <CommunityBadge community/>
                </div>
                <Markdown content/>
                <div class="flex flex-row">
                    {saved.then(|| "saved")} " " {} " " <RelativeTime time=published/> {updated
                        .map(|u| {
                            view! { cx, <RelativeTime time=u/> }
                        })}
                </div>
                <div class="m-5">
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
    let post_comments = create_resource(
        cx,
        move || sort(),
        move |sort| async move {
            let client = use_context::<CapyClient>(cx).unwrap();
            let comments = client
                .execute(GetComments {
                    post_id: Some(post_id),
                    sort: Some(sort),
                    limit: None,
                    page: None,
                    ..Default::default()
                })
                .await
                .unwrap();
            comments
        },
    );

    view! { cx,
        <Suspense fallback=move || {
            view! { cx, "Loading" }
        }>
            {move || {
                post_comments
                    .read(cx)
                    .map(|comments| {
                        let comments = CommentWithChildren::from_comments(comments.comments);
                        comments
                            .into_iter()
                            .map(|comment| {
                                view! { cx, <Comment comment=comment/> }
                            })
                            .collect::<Vec<_>>()
                    })
            }}
        </Suspense>
    }
}
