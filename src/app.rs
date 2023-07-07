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
use leptos_icons::{BiIcon, Icon};
use leptos_meta::*;
use leptos_router::*;
use log::info;
use serde::{Deserialize, Serialize};
use std::{ops::Deref, panic};
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

#[derive(Clone, Copy)]
pub struct CurrentUser(pub RwSignal<Option<LoginInfo>>);

impl Deref for CurrentUser {
    type Target = RwSignal<Option<LoginInfo>>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

#[component]
pub fn App(cx: Scope) -> impl IntoView {
    panic::set_hook(Box::new(console_error_panic_hook::hook));
    wasm_logger::init(wasm_logger::Config::default());
    let jwt = Settings::current_login();
    let current_user = CurrentUser(create_rw_signal(cx, jwt.clone()));
    info!("{jwt:?}");
    provide_context(cx, current_user);
    provide_context(
        cx,
        CapyClient::new(
            current_user
                .0
                .get_untracked()
                .map(|u| u.instance.to_string())
                .unwrap_or("https://lemmy.world".to_string()),
            current_user.0.get_untracked().map(|user| user.jwt.clone()),
        ),
    );
    create_effect(cx, move |_| {
        let user = current_user();
        Settings::set_current_login(user.clone());
        let client = use_context::<CapyClient>(cx).unwrap();
        if let Some(user) = &user {
            client.set_instance(user.instance.to_string());
        }
        client.set_jwt(user.map(|u| u.jwt));
    });
    // keeps a unique key to refresh the user list
    let user_list = create_rw_signal(cx, 0);
    view! { cx,
        <Body class="bg-neutral-100 dark:bg-neutral-900 text-base dark:text-white"/>
        <main class="container mx-auto px-4">
            <div class="flex flex-row gap-2">
                <a href="/">"home"</a>
                <a href="/login">"Login"</a>
                <a href="/communities">"Communities"</a>
                <Profile/>
                {move || {
                    let mut logins = Settings::get_logins();
                    let user = current_user();
                    logins.retain(|l| !user.as_ref().map(|r| r == l).unwrap_or_default());
                    logins
                        .into_iter()
                        .map(|login| Some(login))
                        .chain([None].into_iter())
                        .map(|login| {
                            let login_value = login.clone();
                            let login_value_2 = login.clone();
                            view! { cx,
                                <button
                                    class="bg-neutral-800 p-1 rounded hover:bg-neutral-500"
                                    on:click=move |_| {
                                        current_user.set(login_value.clone());
                                    }
                                >
                                    {if let Some(login) = login {
                                        view!{cx, {login.username}
                                        "@"
                                        {login.instance}}.into_view(cx)
                                    } else {
                                        "guest".into_view(cx)
                                    }}
                                </button>
                                {login_value_2.map(|login| {
                                    view!{cx, <button class="bg-neutral-800 rounded p-1 hover:bg-neutral-500"
                                    on:click=move |_| {
                                        Settings::remove_login(login.clone());
                                        user_list.update(|i| *i += 1);
                                    }>
                                    <Icon icon=MaybeSignal::Static(BiIcon::BiLogOutRegular.into()) />
                                </button>}
                                })}

                            }
                        })
                        .collect::<Vec<_>>()
                }}
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
                        path="/communities"
                        view=move |cx| {
                            view! { cx, <CommunityList/> }
                        }
                    />
                    <Route
                        path="/c/:community"
                        view=move |cx| {
                            view! { cx, <Community/> }
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
    let user = use_context::<CurrentUser>(cx).unwrap();
    let posts = create_local_resource(
        cx,
        move || (sort(), type_(), user.0()),
        move |(sort, type_, _)| async move {
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
                <SortMenu sort set_sort/>
                <TypeMenu type_ set_type/>
            </div>
            <Suspense fallback=move || {
                view! { cx, "Loading" }
            }>
                {move || {
                    posts
                        .with(
                            cx,
                            move |p| {
                                view! { cx,
                                    <ErrorView
                                        value=p.clone()
                                        ok=move |p| {
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
                                            }
                                        }
                                    />
                                }
                            },
                        )
                }}
            </Suspense>
        </div>
    }
}
