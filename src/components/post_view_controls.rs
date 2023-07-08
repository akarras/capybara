use leptos::*;

use crate::components::feed::post_preview::ViewMode;

use super::feed::post_preview::{GlobalBlurState, GlobalViewMode};

#[component]
pub fn PostViewControls(cx: Scope) -> impl IntoView {
    let blur_nsfw = use_context::<GlobalBlurState>(cx).unwrap();
    let post_view_mode = use_context::<GlobalViewMode>(cx).unwrap();
    view! {cx,
    <div class="flex flex-row">
        <input type="checkbox" id="blurnsfw" name="blurnsfw" value="NSFW" prop:checked=blur_nsfw.0 on:click=move |_| {
            blur_nsfw.0.update(|nsfw| {*nsfw = !*nsfw;});
        }/>
        <label for="blurnsfw">"blur nsfw:"</label>
        <button class="bg-neutral-800 hover:bg-neutral-500 border-gray-300 border-b-1 p-1 align-bottom" on:click=move |_| { post_view_mode.0.set(ViewMode::Default) } >
            "default"
        </button>
        <button class="bg-neutral-800 hover:bg-neutral-500 border-gray-300 border-b-1 p-1 align-bottom" on:click=move |_| { post_view_mode.0.set(ViewMode::BigImage) } >
            "big images"
        </button>
    </div>}
}
