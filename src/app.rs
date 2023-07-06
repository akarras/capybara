use crate::{
    community::Community,
    community_list::{CommunityList, CommunityView},
    components::{
        feed::{post_preview::*, virtual_scroll::InfinitePage},
        post::Post,
        profile::Profile,
        sorting_components::{SortMenu, TypeMenu},
    },
    login::Login,
    settings::{LoginInfo, Settings},
};
use capybara_lemmy_client::{
    post::{GetPosts, PostView},
    sensitive::Sensitive,
    CapyClient,
};
use leptos::*;
use leptos_meta::*;
use leptos_router::*;
use log::info;
use serde::{Deserialize, Serialize};
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

#[derive(Clone)]
pub struct CurrentUser(pub RwSignal<Option<LoginInfo>>);

#[component]
pub fn App(cx: Scope) -> impl IntoView {
    panic::set_hook(Box::new(console_error_panic_hook::hook));
    wasm_logger::init(wasm_logger::Config::default());
    let current_user = Settings::current_login();
    let jwt = CurrentUser(create_rw_signal(cx, current_user.clone()));
    info!("{current_user:?}");
    provide_context(cx, jwt);
    provide_context(
        cx,
        CapyClient::new(
            current_user
                .as_ref()
                .map(|u| u.instance.to_string())
                .unwrap_or("https://lemmy.world".to_string()),
            current_user.map(|user| user.jwt.clone()),
        ),
    );
    view! { cx,
        <Body class="bg-neutral-100 dark:bg-neutral-900 text-base dark:text-white"/>
        <main class="container mx-auto px-4">
            <div class="flex flex-row gap-2">
                <a href="/">"home"</a>
                <a href="/login">"Login"</a>
                <a href="/communities">"Communities"</a>
                <Profile />
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
                    <Route path="/communities" view=move |cx| {view!{cx, <CommunityList />}} />
                    <Route path="/c/:community" view=move |cx| {view!{cx, <Community />}} />
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
pub fn ErrorView<Draw, DrawView, T>(
    cx: Scope,
    value: Result<T, leptos::error::Error>,
    ok: Draw,
) -> impl IntoView
where
    DrawView: IntoView + 'static,
    T: 'static + Clone,
    Draw: Fn(T) -> DrawView + 'static,
{
    {
        match value {
            Ok(o) => ok(o.clone()).into_view(cx),
            Err(e) => format!("Error!\n{e}").into_view(cx),
        }
    }
    .into_view(cx)
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

            client
                .execute(GetPosts {
                    sort,
                    type_,
                    ..Default::default()
                })
                .await
                .map_err(leptos::error::Error::from)
        },
    );
    view! { cx,
        <div class="flex flex-col">
            <div class="flex flex-row sticky h-10">
                <SortMenu sort set_sort />
                <TypeMenu type_ set_type />
            </div>
            <Suspense fallback=move || {
                view! { cx, "Loading" }
            }>
                {move || {

                    posts
                        .with(cx, move |p| {
                            view!{cx, <ErrorView value=p.clone() ok=move |p| {
                                let posts = p.posts;
                                let sort = sort();
                                let type_ = type_();
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
                                    />
                                }}/>}
                            })
                }}
            </Suspense>
        </div>
    }
}
