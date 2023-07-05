use capybara_lemmy_client::{site::GetSite, CapyClient};
use leptos::*;
use log::info;

use crate::app::{CurrentUser, ErrorView};

#[component]
pub(crate) fn Profile(cx: Scope) -> impl IntoView {
    let user = use_context::<CurrentUser>(cx).unwrap();
    let resource = create_local_resource(
        cx,
        move || user.0(),
        move |_user| async move {
            let client = use_context::<CapyClient>(cx).unwrap();
            let get_site = GetSite::default();
            client
                .execute(get_site)
                .await
                .map_err(leptos::error::Error::from)
        },
    );
    view! {cx,
    <div class="bg-gray-700 rounded">
        <Suspense fallback=move || view!{cx, "Loading"}>
            {move || {
                resource.read(cx).map(|profile| {
                    view!{cx,
                        <ErrorView value=profile ok=move |profile| {
                           profile.my_user.map(|profile| {
                               info!("{profile:?}");
                               let person = profile.local_user_view.person;
                               view!{cx,
                                   <div class="flex flex-row">
                                       {person.avatar.map(|url| view!{cx, <img class="rounded-full w-10 h-10" src=url.to_string()/>})}
                                       <div class="text-gray-300">{person.name}</div>
                                   </div>
                               }
                           })
                        } />}
                })
            }}
        </Suspense>
    </div>}
}
