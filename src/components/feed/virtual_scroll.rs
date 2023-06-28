use leptos::{*, html::{Body, body}};
use leptos_use::{use_scroll, UseScrollReturn, use_scroll_with_options, UseScrollOptions, ScrollOffset};
use log::info;
use std::{future::Future, hash::Hash};
use web_sys::HtmlDivElement;

#[component]
pub fn InfinitePage<P, PFut, K, KF, VF, V, T>(
    cx: Scope,
    get_page: P,
    initial_data: Vec<T>,
    key: KF,
    view: VF,
    page_size: usize,
) -> impl IntoView
where
    P: Fn(usize) -> PFut + 'static + Copy,
    PFut: Future<Output = Vec<T>>,
    T: 'static + Clone,
    KF: Fn(&T) -> K + 'static,
    K: Eq + Hash + 'static,
    VF: Fn(Scope, T) -> V + 'static,
    V: IntoView,
{
    let scroller = create_node_ref(cx);
    let data = create_rw_signal(cx, initial_data);
    let (hydrating, set_hydrating) = create_signal(cx, false);
    let current_page = create_rw_signal(cx, 1);
    let UseScrollReturn {
        x,
        set_x,
        y,
        set_y,
        is_scrolling,
        arrived_state,
        directions,
        measure,
    } = use_scroll(cx, scroller);
    let hydrate = move || {
        if !hydrating.get_untracked() {
        set_hydrating(true);
            spawn_local(async move {
                current_page.update(|p| *p += 1);
                let page = current_page.get_untracked();
                let new_data = get_page(page as usize).await;
                data.update(|data| {
                    data.extend(new_data);
                });
                set_hydrating(false);
            });
        }
    };
    let at_bottom = create_memo(cx, move |_| arrived_state().bottom);
    create_effect(cx, move |_| {
        
        info!("{} {} {} {}", hydrating(), at_bottom(), is_scrolling(), y());
        if !hydrating() && at_bottom() {
            hydrate();      
        }
    });
    
    
    view! {cx,
    <div class="max-h-screen overflow-y-auto" node_ref=scroller>
        <For
        each=data
        key
        view
        />

        {move || hydrating().then(|| view!{cx, "Loading!"})}
        {move || (!hydrating()).then(|| view!{cx, <button class="bg-gray-300 rounded px-3" on:click=move |_| {
            hydrate();
        }>"load more"</button>})}
        <button class="bg-gray-700 rounded px-3 sticky bottom-2 right-2" on:click=move |_| {
            set_y(0.0);
        }>"Back to top"</button>
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
