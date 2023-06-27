use capybara_lemmy_client::post::{Post, PostAggregates, PostView};
use leptos::*;

use crate::components::{community::CommunityBadge, person::PersonView};

#[component]
pub fn PostPreview(cx: Scope, post: PostView) -> impl IntoView {
    let PostView {
        post,
        creator,
        community,
        creator_banned_from_community,
        counts,
        subscribed,
        saved,
        read,
        creator_blocked,
        my_vote,
        unread_comments,
    } = post;
    let PostAggregates {
        post_id,
        comments,
        score,
        upvotes,
        downvotes,
        published,
        newest_comment_time_necro,
        newest_comment_time,
        featured_community,
        featured_local,
        hot_rank,
        hot_rank_active,
        ..
    } = counts;
    let Post {
        id,
        name,
        url,
        body,
        creator_id,
        community_id,
        removed,
        locked,
        published,
        updated,
        deleted,
        nsfw,
        embed_title,
        embed_description,
        thumbnail_url,
        ap_id,
        local,
        embed_video_url,
        language_id,
        featured_community,
        featured_local,
    } = post;
    view! { cx,
        <div class="bg-gray-800 hover:bg-gray-700 p-4 m-5">
            <div class="flex flex-row gap-3">
                <PersonView person=creator/>
                "to"
                <CommunityBadge community />
            </div>
            <div class="flex flex-row">
                {name} " " {locked.then(|| "locked")} " " {local.then(|| "local")} " "
                {nsfw.then(|| "nsfw")}
            </div>
            {url
                .map(|url| {
                    view! { cx,
                        <a class="text-gray-500 hover:text-gray-400 underline" href=url.to_string()>
                            {url.to_string()}
                        </a>
                    }
                })}
            <div class="flex-row">
                <a href=format!("/post/{}", id.0)>
                    <div class="text-gray-300">
                        {comments} " comments (" {unread_comments} " unread)"
                    </div>
                </a>
            </div>
            {thumbnail_url
                .map(|url| {
                    view! { cx, <img class="h-96 w-96 object-scale-down" src=url.to_string()/> }
                })}
            {body
                .map(|body| {
                    view! { cx, <div class="prose dark:prose-invert">{body}</div> }
                })}
            {embed_title
                .map(|title| {
                    view! { cx, <div class="text-lg">{title}</div> }
                })}
            {embed_description
                .map(|description| {
                    view! { cx, <div>{description}</div> }
                })}
            {embed_video_url
                .map(|url| {
                    view! { cx, <video class="h-96 w-96 object-scale-down" src=url.to_string()></video> }
                })}
        </div>
    }
}
