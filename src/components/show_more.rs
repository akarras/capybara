use leptos::*;
use leptos_use::{use_element_size, UseElementSizeReturn};

#[component]
pub fn ShowMore(cx: Scope, children: Children) -> impl IntoView {
    let node_ref = create_node_ref(cx);
    let UseElementSizeReturn { height, .. } = use_element_size(cx, node_ref);
    let needs_more = move || height() > 160.0;
    let (show_all, set_show_all) = create_signal(cx, false);
    let _ = view! {cx, <div class="h-40"></div>};
    view! { cx,
        <div class=move || if show_all() || !needs_more() { "" } else { "overflow-clip h-40" }>
            <div node_ref=node_ref>{children(cx)}</div>
        </div>
        {move || {
            (!show_all() && needs_more())
                .then(|| {
                    view! { cx,
                        <div
                            class="p-1 text-red-400 underline hover:text-red-500 text-lg cursor-pointer"
                            on:click=move |_| {
                                set_show_all(true);
                            }
                        >
                            "Show more"
                        </div>
                    }
                })
        }}
    }
}
