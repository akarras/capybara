use std::ops::Deref;

use crate::{
    app::{CurrentUser, HideRead},
    components::{
        community::CommunityBadge, markdown::Markdown, person::PersonView, show_more::ShowMore,
        time::RelativeTime, voter::Voter,
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
                let _ = player.play();
            } else {
                let _ = player.pause();
            }
        }
    });
    let global_view_mode = use_context::<GlobalViewMode>(cx).unwrap();

    view! {cx, <video controls node_ref=video_player class=move || match global_view_mode.0()  { ViewMode::BigImage => "min-h-96 min-w-96 max-h-[calc(100vh-200px)] max-w-full aspect-video", ViewMode::Default => "h-96 w-fit aspect-video" } src=src />}
}

#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub enum ViewMode {
    Default,
    BigImage,
}

#[derive(Copy, Clone)]
pub struct GlobalViewMode(pub RwSignal<ViewMode>);

impl AsRef<RwSignal<ViewMode>> for GlobalViewMode {
    fn as_ref(&self) -> &RwSignal<ViewMode> {
        &self.0
    }
}

#[derive(Copy, Clone)]
pub struct GlobalBlurState(pub RwSignal<bool>);

impl AsRef<RwSignal<bool>> for GlobalBlurState {
    fn as_ref(&self) -> &RwSignal<bool> {
        &self.0
    }
}

#[component]
pub fn PostPreview(cx: Scope, post: PostView) -> impl IntoView {
    let view_mode = use_context::<GlobalViewMode>(cx).unwrap();
    let global_blur = use_context::<GlobalBlurState>(cx).unwrap();
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
    let subscribed = create_rw_signal(cx, subscribed);
    // get the score without our vote so we can immediately update the signals locally
    let my_vote = create_rw_signal(cx, my_vote);
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
    let hide_read = use_context::<HideRead>(cx).unwrap();
    view! { cx,
        <div class="flex flex-row bg-neutral-900 hover:border-neutral-700 p-1 border-neutral-500 border-b-4" class:hidden=move || read && hide_read.0() >
            <Voter my_vote upvotes downvotes score />
            <div class="flex flex-col">
                <div class="flex flex-row gap-1">
                    <PersonView person=creator/>
                    "to"
                    <CommunityBadge community subscribed />
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
                    {read.then(|| view!{cx, <div class="px-2 rounded bg-gray-800">"read"</div>})}
                </div>
                <div class="flex flex-row">
                    <div class="text-lg">{name}</div>
                </div>
                <div class="blur hidden"></div>
                <div class:blur=move || { nsfw && global_blur.0() } class="hover:blur-none">
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
                                    lazy="true"
                                    on:click=move |_| set_expanded(!expanded())
                                    class=move || {
                                        if !expanded() && view_mode.0() == ViewMode::Default {
                                            "max-h-96 max-w-96 object-scale-down"
                                        } else {
                                            "max-h-[calc(100vh-200px)] max-w-full min-h-96 min-w-96 object-scale-down"
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
                            .map(|mut url| {
                                match url.host_str(){ Some("yewtu.be" | "youtube.com") =>{url.query_pairs_mut().append_pair("autoplay", "0");} _ => {} }
                                let src = url.to_string();
                                if src.contains(".mp4") {
                                    view!{cx, <VideoPlayer src/>}.into_view(cx)
                                } else {

                                    view! { cx, <iframe lazy="true" sandbox="allow-scripts allow-same-origin" allowfullscreen
                                    frameborder="0" class="h-96 w-fit aspect-video" src=src></iframe> }.into_view(cx)
                                }

                            })}
                        {url.map(|u| {let u = u.to_string();
                            is_video(&u).then(||
                                view!{cx, <VideoPlayer src=u />
                            })})}
                    </div>
                </div>
                <div class="flex flex-row gap-2 p-1 leading-none">
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
                    <button class="text-gray-500 hover:text-gray-400 flex flex-row gap-1 underline" on:click=move |_| {
                        let client = use_context::<CapyClient>(cx).unwrap();
                        let current_instance = client.get_instance();
                        let share_url = format!("{current_instance}/post/{}", post_id.0);
                        let clipboard = window().navigator().clipboard().unwrap();
                        let promise = clipboard.write_text(&share_url);
                        spawn_local(async move {
                            wasm_bindgen_futures::JsFuture::from(promise).await.unwrap();
                        })
                    }><Icon icon=MaybeSignal::Static(BsIcon::BsShareFill.into())/>"share"</button>
                </div>
            </div>
        </div>
    }
}
