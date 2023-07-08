use gloo::storage::{SessionStorage, Storage};
use leptos::*;
use leptos_use::{use_scroll_with_options, ScrollOffset, UseScrollOptions, UseScrollReturn};
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
    initial_data: Vec<T>,
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

    let scroller = create_node_ref(cx);
    let data = create_rw_signal(cx, initial_data);
    let (hydrating, set_hydrating) = create_signal(cx, false);
    let current_page = create_rw_signal(cx, 1);
    let (at_end, set_at_end) = create_signal(cx, false);
    let UseScrollReturn {
        set_y,
        arrived_state,
        y,
        ..
    } = use_scroll_with_options(
        cx,
        scroller,
        UseScrollOptions::default().offset(ScrollOffset {
            top: 0.0,
            bottom: 1000.0,
            right: 0.0,
            left: 0.0,
        }),
    );
    let set_y = Rc::new(set_y);
    let set_y_2 = set_y.clone();
    if let Ok(previous_session) = SessionStorage::get::<ScrollerData<T>>(cache_key.to_string()) {
        let ScrollerData {
            data: prev_data,
            y_scroll,
        } = previous_session;
        data.update(|d| *d = prev_data);
        request_animation_frame(move || {
            set_y_2(y_scroll);
        });
        info!("restored previous scrolling list {y_scroll}");
    }
    on_cleanup(cx, move || {
        let y_scroll = y();
        let data = data();
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
    let at_bottom = create_memo(cx, move |_| arrived_state().bottom);
    create_effect(cx, move |_| {
        if !hydrating() && at_bottom() {
            hydrate();
        }
    });

    view! {cx,
    <div class="max-h-screen flex-auto overflow-y-auto h-[calc(100vh-74px)]" node_ref=scroller>
        <button class="bg-gray-700 px-2 text-lg absolute bottom-10 right-10 rounded-md" on:click=move |_| {
            set_y(0.0);
        }>"Back to top"</button>
        <For
        each=data
        key
        view
        />

        {move || hydrating().then(|| view!{cx, "Loading!"})}
        {move || (!hydrating() && !at_end()).then(|| view!{cx, <button class="bg-gray-300 rounded px-3" on:click=move |_| {
            hydrate();
        }>"load more"</button>})}
        {move || at_end().then(|| view!{cx, "Wow, you're at the end!"})}

    </div>}
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
