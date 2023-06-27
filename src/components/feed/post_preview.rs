use capybara_lemmy_client::{
    person::Person,
    post::{Post, PostAggregates, PostView},
};
use leptos::*;

#[component]
fn PersonView(cx: Scope, person: Person) -> impl IntoView {
    let Person {
        name,
        avatar,
        banned,
        local,
        updated,
        actor_id,
        admin,
        deleted,
        instance_id,
        id,
        ..
    } = person;
    view! {cx, <a class="flex flex-row underline text-red-400" href=format!("/person/{}", id.0)>
        {avatar.map(|a| view!{cx, <img class="w-8 h-8 rounded-full" src=a.to_string() />})}
        <div>{name}</div>
    </a>}
}

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
    view! {
        cx,
        <div class="bg-gray-800 hover:bg-gray-300">
            <PersonView person=creator/>
                <div class="flex flex-row">{name}" "{locked.then(|| "locked")}" "{local.then(|| "local")}" "{nsfw.then(|| "nsfw")}</div>
                {url.map(|url| view!{cx, <a class="text-gray-700 hover:text-gray-400 underline" href=url.to_string()>{url.to_string()}</a>})}
                <div class="flex-row">
                    <div></div>
                    <a href=format!("/post/{}", id.0)><div class="text-gray-300">{comments}" comments"</div></a>
                </div>
                {thumbnail_url.map(|url| view!{cx, <img class="h-96 w-96 object-scale-down" src=url.to_string() /> })}
                {body.map(|body| view!{cx, <div class="prose dark:prose-invert">{body}</div>})}
        </div>
    }
}
