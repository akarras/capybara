use crate::components::{
    feed::{post_preview::*, virtual_scroll::InfinitePage},
    post::Post,
};
use capybara_lemmy_client::{
    post::{GetPost, GetPosts, ListingType, PostView, SortType},
    CapyClient,
};
use leptos::leptos_dom::ev::SubmitEvent;
use leptos::*;
use leptos_meta::*;
use leptos_router::*;
use serde::{Deserialize, Serialize};
use serde_wasm_bindgen::to_value;
use std::panic;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = ["window", "__TAURI__", "tauri"])]
    async fn invoke(cmd: &str, args: JsValue) -> JsValue;
}

#[derive(Serialize, Deserialize)]
struct GreetArgs<'a> {
    name: &'a str,
}

#[component]
pub fn App(cx: Scope) -> impl IntoView {
    panic::set_hook(Box::new(console_error_panic_hook::hook));
    wasm_logger::init(wasm_logger::Config::default());
    provide_context(cx, CapyClient::new("https://lemmy.world"));
    view! { cx,
        <Body class="bg-neutral-100 dark:bg-neutral-900 text-base dark:text-white"/>
        <main class="container mx-auto px-4">
            <div class="flex flex-row gap-2">
                <a href="/">"home"</a>
                <a href="/login">"Login"</a>
            </div>
            <Router>
                <Routes>
                    <Route
                        path="/login"
                        view=move |cx| {
                            view! { cx, <Login/> }
                        }
                    />
                    <Route
                        path="/post/:id"
                        view=move |cx| {
                            view! { cx, <Post/> }
                        }
                    />
                    <Route
                        path="/"
                        view=move |cx| {
                            view! { cx, <Posts/> }
                        }
                    />
                    <Route
                        path=""
                        view=move |_cx| {
                            view! { cx, "404 not found" }
                        }
                    />
                </Routes>
            </Router>
        </main>
    }
}

#[component]
fn PostSort(
    cx: Scope,
    value: Option<SortType>,
    text: &'static str,
    set_sort: WriteSignal<Option<SortType>>,
) -> impl IntoView {
    view! { cx,
        <button
            class="p-2 rounded bg-red-600 hover:bg-red-800"
            on:click=move |_| {
                set_sort(value);
            }
        >
            {text}
        </button>
    }
}

#[component]
fn SortMenu(
    cx: Scope,
    sort: ReadSignal<Option<SortType>>,
    set_sort: WriteSignal<Option<SortType>>,
) -> impl IntoView {
    let sort_menu_hidden = create_rw_signal(cx, true);
    view! { cx,
        <div>
            <button
            class="p-2 rounded bg-red-600 hover:bg-red-800"
            on:click=move |_| {
                sort_menu_hidden.update(|u| *u = !*u);
            }
        >
            "sort: "
            {move || format!("{:?}", sort())}
        </button>
        <div class="flex flex-col absolute z-10" class:hidden=sort_menu_hidden>
            <PostSort value=None text="None" set_sort/>
            <PostSort value=Some(SortType::Active) text="Active" set_sort/>
            <PostSort value=Some(SortType::Hot) text="Hot" set_sort/>
            <PostSort value=Some(SortType::New) text="New" set_sort/>
            <PostSort value=Some(SortType::Old) text="Old" set_sort/>
            <PostSort value=Some(SortType::TopDay) text="Top Day" set_sort/>
            <PostSort value=Some(SortType::TopWeek) text="Top Week" set_sort/>
            <PostSort value=Some(SortType::TopMonth) text="Top Month" set_sort/>
            <PostSort value=Some(SortType::TopYear) text="Top Year" set_sort/>
            <PostSort value=Some(SortType::TopAll) text="Top All" set_sort/>
            <PostSort value=Some(SortType::MostComments) text="Most Comments" set_sort/>
            <PostSort value=Some(SortType::NewComments) text="New Comments" set_sort/>
            <PostSort value=Some(SortType::TopHour) text="Top Hour" set_sort/>
            <PostSort value=Some(SortType::TopSixHour) text="Top 6 Hour" set_sort/>
            <PostSort value=Some(SortType::TopTwelveHour) text="Top 12 Hour" set_sort/>
        </div>
        </div>
    }
}

#[component]
fn TypeMenu(
    cx: Scope,
    type_: ReadSignal<Option<ListingType>>,
    set_type: WriteSignal<Option<ListingType>>,
) -> impl IntoView {
    let options = [
        (Some(ListingType::All), "All"),
        (Some(ListingType::Local), "Local"),
        (Some(ListingType::Subscribed), "Subscribed"),
    ];
    view! { cx,
        <div class="flex-row">
            {options.into_iter().map(|(listing_type, name)| {
                view!{cx, <button
                    class:underline=move || type_() == listing_type
                    class="p-2 rounded bg-red-600 hover:bg-red-800"
                    on:click=move |_| {
                        set_type.set(listing_type);
                    }
                >
                    {name}
                </button>}
            }).collect::<Vec<_>>()}

        </div>
    }
}

#[component]
fn Posts(cx: Scope) -> impl IntoView {
    let (sort, set_sort) = create_signal(cx, None);
    let (type_, set_type) = create_signal(cx, None);
    let posts = create_local_resource(
        cx,
        move || (sort(), type_()),
        move |(sort, type_)| async move {
            let client = use_context::<CapyClient>(cx).expect("need client");
            Some(
                client
                    .execute(GetPosts {
                        sort,
                        type_,
                        ..Default::default()
                    })
                    .await
                    .unwrap(),
            )
        },
    );
    view! { cx,
        <div>
            <div class="flex flex-row">
                <SortMenu sort set_sort />
                <TypeMenu type_ set_type />
            </div>
            <Suspense fallback=move || {
                view! { cx, "Loading" }
            }>
                {move || {
                    let sort = sort();
                    let type_ = type_();
                    posts
                        .read(cx)
                        .map(|post| {
                            post.map(|p| {
                                let posts = p.posts;
                                let page_size = posts.len();
                                view! { cx,
                                    <InfinitePage
                                        get_page=move |page| async move {
                                            let client = use_context::<CapyClient>(cx).expect("need client");
                                            client
                                                .execute(GetPosts {
                                                    page: Some(page as i64),
                                                    type_,
                                                    sort,
                                                    ..Default::default()
                                                })
                                                .await
                                                .unwrap()
                                                .posts
                                        }
                                        key=move |p: &PostView| p.post.id
                                        view=move |cx, post| {
                                            view! { cx, <PostPreview post/> }
                                        }
                                        initial_data=posts
                                        page_size
                                    />
                                }
                            })
                        })
                }}
            </Suspense>
        </div>
    }
}

#[component]
fn Login(cx: Scope) -> impl IntoView {
    let username = create_rw_signal(cx, "".to_string());
    let password = create_rw_signal(cx, "".to_string());
    let instance = create_rw_signal(cx, "".to_string());
    view! { cx,
        <div>
            <form>
                <label for="username">"username:"</label>
                <input id="username"/>
                <label for="password">"password:"</label>
                <input id="password"/>
                <label for="instance">"instance:"</label>
                <input id="instance" on:input=move |_| {}/>
                <button on:click=move |_| {}>"login"</button>
            </form>
        </div>
    }
}

#[component]
fn TauriDemo(cx: Scope) -> impl IntoView {
    let (name, set_name) = create_signal(cx, String::new());
    let (greet_msg, set_greet_msg) = create_signal(cx, String::new());

    let update_name = move |ev| {
        let v = event_target_value(&ev);
        set_name.set(v);
    };

    let greet = move |ev: SubmitEvent| {
        ev.prevent_default();
        spawn_local(async move {
            if name.get().is_empty() {
                return;
            }

            let args = to_value(&GreetArgs { name: &name.get() }).unwrap();
            // Learn more about Tauri commands at https://tauri.app/v1/guides/features/command
            let new_msg = invoke("greet", args).await.as_string().unwrap();
            set_greet_msg.set(new_msg);
        });
    };
    view! { cx,
        <div class="row">
            <a href="https://tauri.app" target="_blank">
                <img src="public/tauri.svg" class="logo tauri" alt="Tauri logo"/>
            </a>
            <a href="https://docs.rs/leptos/" target="_blank">
                <img src="public/leptos.svg" class="logo leptos" alt="Leptos logo"/>
            </a>
        </div>
        <p>"Click on the Tauri and Leptos logos to learn more."</p>
        <p>
            "Recommended IDE setup: " <a href="https://code.visualstudio.com/" target="_blank">
                "VS Code"
            </a> " + " <a href="https://github.com/tauri-apps/tauri-vscode" target="_blank">
                "Tauri"
            </a> " + " <a href="https://github.com/rust-lang/rust-analyzer" target="_blank">
                "rust-analyzer"
            </a>
        </p>
        <form class="row" on:submit=greet>
            <input id="greet-input" placeholder="Enter a name..." on:input=update_name/>
            <button type="submit">"Greet"</button>
        </form>
        <p>
            <b>{move || greet_msg.get()}</b>
        </p>
    }
}
