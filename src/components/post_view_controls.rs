use leptos::*;

use crate::{components::feed::post_preview::ViewMode, app::HideRead};

use super::feed::post_preview::{GlobalBlurState, GlobalViewMode};

#[component]
pub fn PostViewControls(cx: Scope) -> impl IntoView {
    let blur_nsfw = use_context::<GlobalBlurState>(cx).unwrap();
    let post_view_mode = use_context::<GlobalViewMode>(cx).unwrap();
    let hide_read = use_context::<HideRead>(cx).unwrap();
    view! {cx,
    <div class="flex flex-row gap-2">
        <div class="flex flex-row">
            <label for="blurnsfw">"blur nsfw:"</label>
            <input type="checkbox" id="blurnsfw" name="blurnsfw" value="NSFW" prop:checked=blur_nsfw.0 on:click=move |_| {
                blur_nsfw.0.update(|nsfw| {*nsfw = !*nsfw;});
            }/>
        </div>
        <div class="flex flex-row">
            <label for="hideread">"hide read:"</label>
            <input type="checkbox" id="hideread" name="hideread" value="Hide Read" prop:checked=hide_read.0 on:click=move |_| {
                hide_read.0.update(|read| {*read = !*read;});
            }/>
        </div>
        <div class="flex flex-row">
            <button class="bg-neutral-800 hover:bg-neutral-500 border-gray-300 border-b-1 p-1 align-bottom" on:click=move |_| { post_view_mode.0.set(ViewMode::Default) } >
                "default"
            </button>
            <button class="bg-neutral-800 hover:bg-neutral-500 border-gray-300 border-b-1 p-1 align-bottom" on:click=move |_| { post_view_mode.0.set(ViewMode::BigImage) } >
                "big images"
            </button>
        </div>
    </div>}
}
