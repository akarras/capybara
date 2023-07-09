use leptos::*;
use leptos_icons::{BiIcon, Icon};

#[component]
pub fn SaveButton(
    cx: Scope,
    saved: ReadSignal<bool>,
    set_saved: WriteSignal<bool>,
) -> impl IntoView {
    view! { cx,
        <button
            class=move || {
                if saved() {
                    "flex flex-row bold text-yellow-500 hover:text-yellow-400 align-center leading-none"
                } else {
                    "flex flex-row bold text-gray-500 hover:text-gray-400 align-center leading-none"
                }
            }
            on:click=move |_| { set_saved(!saved()) }
        >
            <Icon icon=MaybeSignal::Static(BiIcon::BiSaveRegular.into())/>
            {move || if saved() { "saved" } else { "save" }}
        </button>
    }
}
