use gloo::{
    storage::{SessionStorage, Storage},
    utils::{body, document},
};
use leptos::*;
use leptos_use::{
    use_element_size, use_scroll_with_options, use_window_scroll, ScrollOffset, UseScrollOptions,
    UseScrollReturn,
};
use log::info;
use serde::{de::DeserializeOwned, Deserialize, Serialize};
use std::{
    collections::{hash_map::DefaultHasher, HashSet},
    future::Future,
    hash::{Hash, Hasher},
    rc::Rc,
};
use web_sys::HtmlDivElement;

#[derive(Serialize, Deserialize)]
struct ScrollerData<T> {
    y_scroll: f64,
    data: Vec<T>,
}

#[component]
pub fn InfinitePage<P, PFut, K, KF, VF, V, T, CK>(
    cx: Scope,
    get_page: P,
    data: RwSignal<Vec<T>>,
    key: KF,
    view: VF,
    cache_key: CK,
) -> impl IntoView
where
    P: Fn(usize) -> PFut + 'static + Copy,
    PFut: Future<Output = Vec<T>>,
    T: 'static + Clone + DeserializeOwned + Serialize,
    KF: Fn(&T) -> K + 'static + Copy,
    K: Eq + Hash + 'static,
    VF: Fn(Scope, T) -> V + 'static,
    V: IntoView,
    CK: Hash + Eq + PartialEq + 'static,
{
    let mut hasher = DefaultHasher::new();
    cache_key.hash(&mut hasher);
    let cache_key = hasher.finish();

    // let scroller = create_node_ref(cx);
    let (hydrating, set_hydrating) = create_signal(cx, false);
    let current_page = create_rw_signal(cx, 1);
    let (at_end, set_at_end) = create_signal(cx, data.get_untracked().is_empty());
    if let Ok(previous_session) = SessionStorage::get::<ScrollerData<T>>(cache_key.to_string()) {
        let ScrollerData {
            data: prev_data,
            y_scroll,
        } = previous_session;
        data.update(|d| *d = prev_data);
        request_animation_frame(move || {
            window().scroll_to_with_x_and_y(0.0, y_scroll);
        });
        info!("restored previous scrolling list {y_scroll}");
    }
    let (_window_x, y_scroll) = use_window_scroll(cx);
    on_cleanup(cx, move || {
        let y_scroll = y_scroll.get_untracked();
        let data = data.get_untracked();
        let _ = SessionStorage::set(cache_key.to_string(), ScrollerData { y_scroll, data });
    });
    let hydrate = move || {
        if !hydrating.get_untracked() && !at_end.get_untracked() {
            set_hydrating(true);
            spawn_local(async move {
                current_page.update(|p| *p += 1);
                let page = current_page.get_untracked();
                let new_data = get_page(page as usize).await;
                if new_data.is_empty() {
                    set_at_end(true);
                }
                data.update(|data| {
                    data.extend(new_data);
                    let mut dedup = HashSet::new();
                    data.retain(|post| dedup.insert(key(post)));
                });
                set_hydrating(false);
            });
        }
    };

    let at_top = move || y_scroll() <= 100.0;
    let at_bottom = create_memo(cx, move |_| {
        let scroll_height = if let Some(scrolling_element) = document().scrolling_element() {
            scrolling_element.scroll_height()
        } else {
            body().scroll_height()
        };

        info!("y_scroll: {} height: {}", y_scroll(), scroll_height);
        y_scroll() >= (scroll_height - 1500) as f64
    });
    create_effect(cx, move |_| {
        if at_bottom() {
            hydrate();
        }
    });
    // refresh effect
    create_effect(cx, move |_| {
        if y_scroll() < -30.0 && !hydrating() {
            data.update(|d| d.clear());
            current_page.set(0);
            hydrate();
        }
    });
    // let is_active = intersections.is_active;
    create_effect(cx, move |_| {
        info!(
            "hydrating: {} at_end: {} at_top: {} y: {} at_bottom: {}",
            hydrating(),
            at_end(),
            at_top(),
            y_scroll(),
            at_bottom()
        );
        if !hydrating() && at_end() {
            hydrate();
        }
    });

    view! {cx,

        <button class="bg-gray-700 px-2 text-lg rounded-md" on:click=move |_| {
            data.update(|d| d.clear());
            current_page.set(0);
            hydrate();
        }>
            "Force Refresh"
        </button>
        <button class:hidden=at_top class="bg-gray-700 px-2 text-lg fixed bottom-10 right-20 rounded-md hover:bg-gray-400 z-50" on:click=move |_| {
            info!("scrolled");
            window().scroll_to_with_x_and_y(0.0, 0.0);
        }>"Back to top"</button>
        <div class="flex flex-col">
            <For
            each=data
            key
            view
            />
        </div>
        {move || hydrating().then(|| view!{cx, "Loading!"})}
        {move || (!hydrating() && !at_end()).then(|| view!{cx, <button class="bg-gray-300 rounded px-3" on:click=move |_| {
            hydrate();
        }>"load more"</button>})}
        {move || (at_end() && !hydrating()).then(|| view!{cx, "Wow, you're at the end!"})}


    }
}

#[component]
pub fn VirtualScroller<T, D, V, KF, K>(
    cx: Scope,
    each: Signal<Vec<T>>,
    key: KF,
    view: D,
    viewport_height: f64,
    row_height: f64,
) -> impl IntoView
where
    D: Fn(Scope, T) -> V + 'static,
    V: IntoView,
    KF: Fn(&T) -> K + 'static,
    K: Eq + Hash + 'static,
    T: 'static + Clone,
{
    let render_ahead = 10;
    let (scroll_offset, set_scroll_offset) = create_signal(cx, 0);
    // use memo here so our signals only retrigger if the value actually changed.
    let child_start = create_memo(cx, move |_| {
        ((scroll_offset() as f64 / row_height) as u32).saturating_sub(render_ahead / 2)
    });
    let children_shown = (viewport_height / row_height).ceil() as u32 + render_ahead;
    create_effect(cx, move |_| {});
    let virtual_children = move || {
        each.with(|children| {
            let array_size = children.len();
            // make sure start + end doesn't go over the length of the vector
            let start = (child_start() as usize).min(array_size);
            let end = (child_start() + children_shown).min(array_size as u32) as usize;
            children[start..end].to_vec()
        })
    };
    view! {cx,
        <div
        on:scroll= move |scroll| {
            let div = event_target::<HtmlDivElement>(&scroll);
            set_scroll_offset(div.scroll_top());
        }
      style=format!(r#"
        height: {}px;
        overflow: auto;
      "#, viewport_height.ceil() as u32)
    >
      <div

        style=move || {
            format!(r#"
          height: {}px;
          overflow: hidden;
          will-change: transform;
          position: relative;
        "#, (each.with(|children| children.len() + render_ahead as usize) as f64 * row_height).ceil() as u32)}
      >
        <div // offset for visible nodes
          style=move || format!("
            transform: translateY({}px);
          ", (child_start() as f64 * row_height) as u32)
        >
        // {move || virtual_children().into_iter().map(|child| view(cx, child)).collect::<Vec<_>>()}
        // For component currently has issues. Possibly
        // https://github.com/leptos-rs/leptos/issues/533
        <For each=virtual_children
         key=key
         view=view
        />
        </div>
      </div>
    </div>
    }
}
