use capybara_lemmy_client::{
    post::{GetPost, GetPosts},
    CapyClient,
};
use leptos::leptos_dom::ev::SubmitEvent;
use leptos::*;
use leptos_router::*;
use serde::{Deserialize, Serialize};
use serde_wasm_bindgen::to_value;
use wasm_bindgen::prelude::*;
use std::panic;

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
        <main class="container">
            <div class="row">
                <a href="/">"router"</a>
                <a href="/login">"Login"</a>
            </div>
            <Router>
                <Routes>
                    <Route path="/" view=Posts />
                    <Route path="/login" view=Login />
                </Routes>
            </Router>
        </main>
    }
}

#[component]
fn Posts(cx: Scope) -> impl IntoView {
    let posts = create_local_resource(
        cx,
        move || {},
        move |_| {
            async move {
                let client = use_context::<CapyClient>(cx).expect("need client");
                Some(client.get_posts(GetPosts {
                    ..Default::default()
                }).await.unwrap())
            }
        },
    );
    view! {cx,
    <div>
        <Suspense fallback=move || view!{cx, "Loading"}>

            {move || {
                posts.read(cx).map(|post| {
                    post.map(|p| {
                        p.posts.into_iter().map(|post| {
                            view!{cx, 
                                <div>{post.post.name}</div>
                            }
                        }).collect::<Vec<_>>()
                    })
                })
            }}
        </Suspense>
    </div>}
}

#[component]
fn Login(cx: Scope) -> impl IntoView {
    let username = create_rw_signal(cx, "".to_string());
    let password = create_rw_signal(cx, "".to_string());
    let instance = create_rw_signal(cx, "".to_string());
    view! {cx,
        <div>
            <form>
                <label for="username">"username:"</label>
                <input id="username" />
                <label for="password">"password:"</label>
                <input id="password" />
                <label for="instance">"instance:"</label>
                <input id="instance" on:input=move |_| {} />
                <button on:click=move |_| {

                }>"login"</button>
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
    view! {cx, <div class="row">
        <a href="https://tauri.app" target="_blank">
            <img src="public/tauri.svg" class="logo tauri" alt="Tauri logo"/>
        </a>
        <a href="https://docs.rs/leptos/" target="_blank">
            <img src="public/leptos.svg" class="logo leptos" alt="Leptos logo"/>
        </a>
    </div>

    <p>"Click on the Tauri and Leptos logos to learn more."</p>

    <p>
        "Recommended IDE setup: "
        <a href="https://code.visualstudio.com/" target="_blank">"VS Code"</a>
        " + "
        <a href="https://github.com/tauri-apps/tauri-vscode" target="_blank">"Tauri"</a>
        " + "
        <a href="https://github.com/rust-lang/rust-analyzer" target="_blank">"rust-analyzer"</a>
    </p>

    <form class="row" on:submit=greet>
        <input
            id="greet-input"
            placeholder="Enter a name..."
            on:input=update_name
        />
        <button type="submit">"Greet"</button>
    </form>

    <p><b>{ move || greet_msg.get() }</b></p>

    }
}
