use capybara_lemmy_client::{
    community::{CommunityId, FollowCommunity, SubscribedType},
    CapyClient,
};
use leptos::*;

use crate::app::CurrentUser;

#[component]
pub fn SubscribeButton(
    cx: Scope,
    community_id: CommunityId,
    subscribed: RwSignal<SubscribedType>,
) -> impl IntoView {
    let (pending, set_pending) = create_signal(cx, false);
    let (error, set_error) = create_signal(cx, None);
    let current_user = use_context::<CurrentUser>(cx).unwrap();
    create_effect(cx, move |prev| {
        let new_sub = subscribed();
        let user = current_user();
        // ensure that we have a user logged in & that the subscription mode changed
        // and ensure that the user is the same
        if let Some((Some(prev_user), prev_sub)) = prev {
            if Some(prev_user) == user && prev_sub != new_sub {
                spawn_local(async move {
                    set_pending(true);
                    let capy_client = use_context::<CapyClient>(cx).unwrap();
                    let follow_req = FollowCommunity {
                        community_id,
                        follow: match new_sub {
                            SubscribedType::Subscribed => true,
                            SubscribedType::NotSubscribed => false,
                            SubscribedType::Pending => true,
                        },
                        ..Default::default()
                    };
                    match capy_client.execute(follow_req).await {
                        Ok(response) => {
                            let view = response.community_view;
                            let new_subscribed = view.subscribed;
                            subscribed.set(new_subscribed);
                        }
                        Err(e) => {
                            set_error(Some(e.to_string()));
                        }
                    }
                    set_pending(false);
                });
            }
        }
        (user, new_sub)
    });
    let _ = view! {cx, <div class="animate-pulse"></div>};
    view! { cx,
        {move || {
            match (current_user(), subscribed()) {
                (None, _) => {
                    view!{cx, <button></button>}
                }
                (_, SubscribedType::Subscribed) => {
                    view! { cx,
                        <button
                            class="p-1 bg-green-600 hover:bg-green-400 underline rounded"
                            on:click=move |_| {
                                subscribed.set(SubscribedType::NotSubscribed);
                            }
                            class:animate-pulse=pending
                        >
                            "Subscribed"
                        </button>
                    }
                }
                (_, SubscribedType::NotSubscribed)=> {
                    view! { cx,
                        <button
                            class="p-1 bg-gray-400 hover:bg-gray-300 underline rounded"
                            on:click=move |_| {
                                subscribed.set(SubscribedType::Subscribed);
                            }
                            class:animate-pulse=pending
                        >
                            "Not Subscribed"
                        </button>
                    }
                }
                (_, SubscribedType::Pending) => {
                    view! { cx,
                        <button
                            class="p-1 bg-yellow-400 bg:text-yellow-300 underline rounded" class:animate-pulse=pending
                            on:click=move |_| {
                                subscribed.set(SubscribedType::Subscribed);
                            }
                        >
                            "Pending"
                        </button>
                    }
                }
            }
        }}
        <div class="text-red-600">{error}</div>
    }
}
