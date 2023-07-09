use crate::app::CurrentUser;
use leptos::*;
use leptos_icons::{BiIcon, Icon};

#[component]
pub fn Voter(
    cx: Scope,
    my_vote: RwSignal<Option<i16>>,
    upvotes: i64,
    downvotes: i64,
    score: i64,
) -> impl IntoView {
    let clean_score = score - my_vote.get_untracked().unwrap_or_default() as i64;
    let clean_downvotes = downvotes - (my_vote.get_untracked().unwrap_or_default() == -1) as i64;
    let clean_upvotes = upvotes - (my_vote.get_untracked().unwrap_or_default() == 1) as i64;
    let upvotes = move || clean_upvotes + (my_vote().unwrap_or_default() == 1) as i64;
    let downvotes = move || clean_downvotes + (my_vote().unwrap_or_default() == -1) as i64;
    let score = move || clean_score + my_vote().unwrap_or_default() as i64;

    view! { cx,
        <div class="flex flex-col h-fit pr-2">
            <button
                class="flex flex-row text-red-400 hover:text-red-600 align-text-top leading-none"
                on:click=move |_| {
                    if my_vote().unwrap_or_default() == 1 {
                        my_vote.set(None);
                    } else {
                        my_vote.set(Some(1))
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
            </button>
            <div class="text-gray-500">{move || score()}</div>
            <button
                class="flex flex-row text-blue-300 hover:text-blue-600 align-text-top leading-none"
                on:click=move |_| {
                    if my_vote().unwrap_or_default() == -1 {
                        my_vote.set(None);
                    } else {
                        my_vote.set(Some(-1))
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
            </button>
        </div>
    }
}
