use capybara_lemmy_client::{
    community::{CommunityId, FollowCommunity, SubscribedType},
    CapyClient,
};
use leptos::*;

#[component]
pub fn SubscribeButton(
    cx: Scope,
    community_id: CommunityId,
    subscribe: SubscribedType,
) -> impl IntoView {
    let (subscribe, set_subscribe) = create_signal(cx, subscribe);
    let (pending, set_pending) = create_signal(cx, false);
    let (error, set_error) = create_signal(cx, None);
    create_effect(cx, move |_| {
        let subscribed = subscribe();
        spawn_local(async move {
            set_pending(true);
            let capy_client = use_context::<CapyClient>(cx).unwrap();
            let follow_req = FollowCommunity {
                community_id,
                follow: match subscribed {
                    SubscribedType::Subscribed => true,
                    SubscribedType::NotSubscribed => false,
                    SubscribedType::Pending => true,
                },
                ..Default::default()
            };
            match capy_client.execute(follow_req).await {
                Ok(response) => {
                    let view = response.community_view;
                    let subscribed = view.subscribed;
                    set_subscribe.set_untracked(subscribed);
                }
                Err(e) => {
                    set_error(Some(e.to_string()));
                }
            }
            set_pending(false);
        });
    });
    view! { cx,
        {move || {
            match subscribe() {
                SubscribedType::Subscribed => {
                    view! { cx,
                        <div
                            class="p-1 text-green-600 underline cursor-pointer"
                            on:click=move |_| {
                                set_subscribe(SubscribedType::NotSubscribed);
                            }
                        >
                            "Subscribed"
                        </div>
                    }
                }
                SubscribedType::NotSubscribed => {
                    view! { cx,
                        <div
                            class="p-1 text-gray-400 underline cursor-pointer"
                            on:click=move |_| {
                                set_subscribe(SubscribedType::Subscribed);
                            }
                        >
                            "Not Subscribed"
                        </div>
                    }
                }
                SubscribedType::Pending => {
                    view! { cx,
                        <div
                            class="p-1 text-yellow-400 stroke-black underline cursor-pointer"
                            on:click=move |_| {
                                set_subscribe(SubscribedType::Subscribed);
                            }
                        >
                            "Pending"
                        </div>
                    }
                }
            }
        }}
        <div class="animate-bounce" prop:hidden=move || !pending()>"loading"</div>
        <div class="text-red-600">{error}</div>
    }
}
