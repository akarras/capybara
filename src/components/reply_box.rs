use capybara_lemmy_client::{
    comment::{CommentId, CreateComment},
    post::PostId,
    CapyClient,
};
use leptos::*;
use leptos_icons::{BsIcon, Icon};

use super::comments::CommentWithChildren;

#[component]
pub fn ReplyButton(
    cx: Scope,
    reply: ReadSignal<bool>,
    set_reply: WriteSignal<bool>,
) -> impl IntoView {
    view! {cx,
        <div
        class=move || {
            if reply() {
                "flex flex-row bold text-yellow-500 hover:text-yellow-400 bg-gray-500 rounded leading-none"
            } else {
                "flex flex-row bold text-gray-500 hover:text-gray-400 leading-none"
            }
        }
        on:click=move |_| {
            set_reply(!reply());
        }
    >
        <Icon icon=MaybeSignal::Static(BsIcon::BsReplyFill.into())/>
        " reply"
    </div>}
}

#[component]
pub fn ReplyBox(
    cx: Scope,
    post_id: PostId,
    parent_id: Option<CommentId>,
    reply: ReadSignal<bool>,
    set_reply: WriteSignal<bool>,
    children: RwSignal<Vec<CommentWithChildren>>,
) -> impl IntoView {
    let (content, set_content) = create_signal(cx, "".to_string());
    view! { cx,
        <div class="flex flex-col" class:hidden=move || !reply()>
            <textarea
                class="h-36 w-[calc(100%-30px)] rounded ring inset-2 ring-neutral-700 focus:ring-neutral-500 bg-neutral-700 text-neutral-100 p-4 m-4"
                on:input=move |i| { set_content(event_target_value(&i)) }
            ></textarea>
            <div class="flex flex-row">
                <button
                    class="bg-gray-600 p-1 rounded hover:bg-gray-300"
                    on:click=move |_| {
                        let request = CreateComment {
                            content: content(),
                            post_id,
                            parent_id,
                            ..Default::default()
                        };
                        set_content("".to_string());
                        set_reply(false);
                        spawn_local(async move {
                            let client = use_context::<CapyClient>(cx).unwrap();
                            if let Ok(response) = client.execute(request).await {
                                children
                                    .update(|c| {
                                        c.push(CommentWithChildren(response.comment_view, vec![]));
                                    });
                            }
                        });
                    }
                >
                    "send reply"
                </button>
            </div>
        </div>
    }
}
