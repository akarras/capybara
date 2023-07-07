use std::ops::Deref;

use crate::{
    app::CurrentUser,
    components::{
        community::CommunityBadge, markdown::Markdown, person::PersonView, show_more::ShowMore,
        time::RelativeTime,
    },
};
use capybara_lemmy_client::{
    post::{CreatePostLike, Post, PostAggregates, PostView},
    CapyClient,
};
use leptos::{html::Video, *};
use leptos_icons::*;
use leptos_use::{
    use_intersection_observer, use_intersection_observer_with_options,
    UseIntersectionObserverOptions,
};
use log::info;
use serde::Serialize;
use wasm_bindgen::JsCast;
use web_sys::HtmlMediaElement;

fn is_magic_embed(url: &str) -> bool {
    url.starts_with("https://redgifs.com/watch") || url.starts_with("https://www.redgifs.com/watch")
}

/// Tries to get an embed from known websites
#[component]
fn MagicEmbed(cx: Scope, url: String) -> impl IntoView {
    if is_magic_embed(&url) {
        let video_id = url.split("/").last();
        video_id.map(|video_id| { view!{cx, <iframe src=format!("https://www.redgifs.com/ifr/{video_id}") frameborder="0" scrolling="no" allowfullscreen class="object-scale-down h-96 aspect-video"></iframe><p><a href=format!("https://www.redgifs.com/watch/{video_id}")>"via RedGIFs"</a></p>}}).into_view(cx)
    } else {
        {}.into_view(cx)
    };
}

fn is_image(url: &str) -> bool {
    url.ends_with(".png")
        || url.ends_with(".webp")
        || url.ends_with(".jpeg")
        || url.ends_with(".jpg")
        || url.ends_with(".gif")
}

fn is_video(url: &str) -> bool {
    url.ends_with(".mp4") || url.ends_with(".webm")
}

#[derive(Serialize)]
enum ImageType {
    #[serde(rename = "webp")]
    Webp,
    #[serde(rename = "jpeg")]
    Jpeg,
    #[serde(rename = "png")]
    Png,
}

#[derive(Serialize)]
struct ImageDetails {
    format: ImageType,
    thumbnail: u32,
}

#[component]
fn LemmyImage(cx: Scope, thumbnail: Option<String>, link: Option<String>) -> impl IntoView {}

#[component]
fn VideoPlayer(cx: Scope, src: String) -> impl IntoView {
    let video_player = create_node_ref(cx);
    let (playback_enabled, set_playback_enabled) = create_signal(cx, false);
    use_intersection_observer_with_options(
        cx,
        video_player,
        move |entry, _| {
            set_playback_enabled(entry[0].is_intersecting());
        },
        UseIntersectionObserverOptions::default().thresholds(vec![0.8]),
    );
    let playback_enabled = create_memo(cx, move |_| playback_enabled());
    create_effect(cx, move |_| {
        let player = video_player();
        let enabled = playback_enabled();
        if let Some(player) = player.and_then(|p: HtmlElement<Video>| {
            let p = p.into_any();
            let cast = p.deref().clone().dyn_into::<HtmlMediaElement>().ok();
            cast
        }) {
            if enabled {
                player.play();
            } else {
                player.pause();
            }
        }
    });
    view! {cx, <video controls node_ref=video_player crossorigin="" class="h-96 w-fit aspect-video" src=src />}
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
    let url_str = url.as_ref().map(|s| s.to_string()).unwrap_or_default();
    let has_embed = embed_title.is_some()
        || embed_description.is_some()
        || embed_video_url.is_some()
        || is_video(&url_str)
        || is_magic_embed(&url_str);
    // get the score without our vote so we can immediately update the signals locally
    let clean_score = score - my_vote.unwrap_or_default() as i64;
    let clean_downvotes = downvotes - (my_vote.unwrap_or_default() == -1) as i64;
    let clean_upvotes = upvotes - (my_vote.unwrap_or_default() == 1) as i64;
    let (my_vote, set_vote) = create_signal(cx, my_vote);
    let upvotes = move || clean_upvotes + (my_vote().unwrap_or_default() == 1) as i64;
    let downvotes = move || clean_downvotes + (my_vote().unwrap_or_default() == -1) as i64;
    let score = move || clean_score + my_vote().unwrap_or_default() as i64;
    let user = use_context::<CurrentUser>(cx).unwrap();
    create_effect(cx, move |prev| {
        let vote = my_vote();
        let user = user();
        if let Some((Some(prev_user), prev)) = prev {
            if Some(prev_user) == user && vote != prev {
                spawn_local(async move {
                    let client = use_context::<CapyClient>(cx).unwrap();
                    let like = CreatePostLike {
                        post_id,
                        score: vote.unwrap_or_default(),
                        ..Default::default()
                    };
                    // TODO: Actually update the post somehow?
                    let _ = client.execute(like).await;
                    info!("liked");
                });
            }
        }
        (user, vote)
    });
    let thumbnail_url = match thumbnail_url {
        Some(t) => Some(t),
        None => match &url {
            Some(url) => {
                let url_str = url.to_string();
                let is_image = is_image(&url_str);
                info!("attempting to detect URL type {url_str} {is_image}");
                if is_image {
                    Some(url.clone())
                } else {
                    None
                }
            }
            None => None,
        },
    };
    view! { cx,
        <div class="flex flex-row bg-neutral-900 hover:border-neutral-700 p-1 border-neutral-500 border-b-4">
            <div class="flex flex-col w-12 h-fit">
                <div
                    class="flex flex-row text-red-400 hover:text-red-600 align-text-top leading-none"
                    on:click=move |_| {
                        if my_vote().unwrap_or_default() == 1 {
                            set_vote(None);
                        } else {
                            set_vote(Some(1))
                        }
                    }
                >
                    {move || {
                        if my_vote() == Some(1) {
                            view! { cx, <Icon icon=MaybeSignal::Static(BiIcon::BiUpvoteSolid.into())/> }
                        } else {
                            view! { cx, <Icon icon=MaybeSignal::Static(BiIcon::BiUpvoteRegular.into())/> }
                        }
                    }}
                    {move || upvotes()}
                </div>
                <div class="text-gray-500">{move || score()}</div>
                <div
                    class="flex flex-row text-blue-300 hover:text-blue-600 align-text-top leading-none"
                    on:click=move |_| {
                        if my_vote().unwrap_or_default() == -1 {
                            set_vote(None);
                        } else {
                            set_vote(Some(-1))
                        }
                    }
                >
                    {move || {
                        if my_vote() == Some(-1) {
                            view! { cx, <Icon icon=MaybeSignal::Static(BiIcon::BiDownvoteSolid.into())/> }
                        } else {
                            view! { cx, <Icon icon=MaybeSignal::Static(BiIcon::BiDownvoteRegular.into())/> }
                        }
                    }}
                    {move || downvotes()}
                </div>
            </div>
            <div class="flex flex-col">
                <div class="flex flex-row gap-1">
                    <PersonView person=creator/>
                    "to"
                    <CommunityBadge community/>
                    {locked
                        .then(|| {
                            view! { cx, <div class="bg-slate-500 px-2 rounded">"locked"</div> }
                        })}
                    {nsfw
                        .then(|| {
                            view! { cx, <div class="bg-red-600 px-2 rounded">"nsfw"</div> }
                        })}
                    <RelativeTime time=published/>
                    {updated
                        .map(|u| {
                            view! { cx,
                                "(updated "
                                <RelativeTime time=u/>
                                ")"
                            }
                        })}
                    {featured_community.then(|| view!{cx, "📌"})}
                    {featured_local.then(|| view!{cx, "📍"})}
                </div>
                <div class="flex flex-row">
                    <div class="text-lg">{name}</div>
                </div>
                <div class="blur hidden"></div>
                <div class:blur=nsfw class="hover:blur-none">
                    {url.as_ref()
                        .map(|url| {
                            view! { cx,
                                <a
                                    target="_blank"
                                    class="text-gray-500 hover:text-gray-400 underline"
                                    href=url.to_string()
                                >
                                    {url.to_string()}
                                </a>
                            }
                        })}
                    {thumbnail_url
                        .map(|url| {
                            let (expanded, set_expanded) = create_signal(cx, false);
                            view! { cx,
                                <img

                                    on:click=move |_| set_expanded(!expanded())
                                    class=move || {
                                        if !expanded() {
                                            "max-h-96 max-w-96 object-scale-down"
                                        } else {
                                            "max-h-[calc(100vh-200px)] max-w-screen min-h-96 min-w-96 object-scale-down"
                                        }
                                    }
                                    src=url.to_string()
                                />
                            }
                        })}
                    {body
                        .map(|body| {
                            view! { cx, <ShowMore><Markdown content=body/></ShowMore> }
                        })}
                    {url.as_ref().map(|url| view!{cx, <MagicEmbed url=url.to_string() />})}
                    <div class="bg-neutral-700 p-1 rounded" class:hidden=!has_embed>
                        {embed_title
                            .map(|title| {
                                view! { cx, <div class="text-md">{title}</div> }
                            })}
                        {embed_description
                            .map(|description| {
                                view! { cx, <div class="text-sm">{description}</div> }
                            })}
                        {embed_video_url
                            .map(|url| {
                                view! { cx, <iframe class="h-96 w-fit aspect-video" src=url.to_string()></iframe> }
                            })}
                        {url.map(|u| {let u = u.to_string();
                            is_video(&u).then(||
                                view!{cx, <VideoPlayer src=u />
                            })})}
                    </div>
                </div>
                <div class="flex-row">
                    <a
                        class="text-gray-500 hover:text-gray-400 underline flex flex-row"
                        href=format!("/post/{}", id.0)
                    >
                        {comments}
                        " comments "
                        {(comments != unread_comments && unread_comments != 0)
                            .then(|| {
                                view! { cx, <div class="text-red-300">"(" {unread_comments} " unread)"</div> }
                            })}
                    </a>
                </div>
            </div>
        </div>
    }
}
