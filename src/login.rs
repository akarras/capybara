use capybara_lemmy_client::{person::Login as LemmyLogin, sensitive::Sensitive, CapyClient};
use leptos::*;
use leptos_router::{use_navigate, NavigateOptions};

use crate::settings::{LoginInfo, Settings};

#[component]
pub(crate) fn Login(cx: Scope) -> impl IntoView {
    let username = create_rw_signal(cx, "".to_string());
    let password = create_rw_signal(cx, "".to_string());
    let instance = create_rw_signal(cx, "https://".to_string());
    let two_factor = create_rw_signal(cx, None);
    view! { cx,
        <div class="p-4 dark:bg-gray-800">
            <div class="flex flex-col">
                <label for="username" class="mb-2">
                    "username:"
                </label>
                <input
                    id="username"
                    class="mb-2 p-2 border border-gray-300 dark:bg-gray-700 dark:border-gray-600 rounded-md"
                    on:input=move |e| username.update(|u| *u = event_target_value(&e))
                />
                <label for="password" class="mb-2">
                    "password:"
                </label>
                <input
                    id="password"
                    class="mb-2 p-2 border border-gray-300 dark:bg-gray-700 dark:border-gray-600 rounded-md"
                    type="password"
                    on:input=move |e| password.update(|p| *p = event_target_value(&e))
                />
                <label for="twofactor" class="mb-2">
                    "totp 2factor token (optional):"
                </label>
                <input
                    id="twofactor"
                    class="mb-2 p-2 border border-gray-300 dark:bg-gray-700 dark:border-gray-600 rounded-md"
                    on:input=move |e| two_factor.update(|p| {
                        let value = event_target_value(&e);
                        *p = (!value.is_empty()).then(|| value);
                    })
                />
                <label for="instance" class="mb-2">
                    "instance:"
                </label>
                <input
                    id="instance"
                    class="mb-2 p-2 border border-gray-300 dark:bg-gray-700 dark:border-gray-600 rounded-md"
                    on:input=move |e| instance.update(|i| *i = event_target_value(&e))
                />
                <button
                    class="p-2 bg-blue-500 dark:bg-blue-700 text-white dark:text-gray-200 rounded-md"
                    on:click=move |e| {
                        let login_request = LemmyLogin {
                            username_or_email: Sensitive::new(username.get_untracked()),
                            password: Sensitive::new(password.get_untracked()),
                            totp_2fa_token: two_factor.get_untracked()
                        };
                        spawn_local(async move {
                            let client = use_context::<CapyClient>(cx).unwrap();
                            client.set_instance(instance.get_untracked());
                            let login = client.execute(login_request).await.unwrap();
                            let jwt = login.jwt.unwrap();
                            let login = LoginInfo {
                                jwt,
                                instance: instance.get_untracked(),
                                username: username.get_untracked()
                            };
                            Settings::create_login(cx, login);
                            log!("logged in!");
                            let navigate = use_navigate(cx);
                            navigate("/", NavigateOptions::default()).unwrap();
                        });
                    }
                >
                    "login"
                </button>
            </div>
        </div>
    }
}
