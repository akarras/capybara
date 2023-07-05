use capybara_lemmy_client::person::Person;
use leptos::*;

#[component]
pub fn PersonView(cx: Scope, person: Person) -> impl IntoView {
    let Person {
        name,
        avatar,
        banned,
        local,
        actor_id,
        admin,
        deleted,
        id,
        ..
    } = person;
    view! { cx,
        <a
            class="flex flex-row underline text-red-400 hover:text-red-600"
            class:bold=admin
            class:line-through=deleted
            href=format!("/person/{}", id.0)
        >
            {avatar
                .map(|a| {
                    view! { cx, <img class="w-6 h-6 rounded-full p-r-2" src=a.to_string()/> }
                })}
            <div class="group font-lg flex flex-row">
                {name} {(!local)
                    .then(|| {
                        view! { cx, <div class="italic">"@" {actor_id.host_str().unwrap_or_default().to_string()}</div>
                        {admin.then(|| view!{cx, <div class="bg-red-700 rounded p-1">"admin"</div>})}
                        {banned.then(|| view!{cx, <div class="bg-red-900 rounded p-1">"banned"</div>})}
                        }
                    })}
            </div>
        </a>
    }
}
