use capybara_lemmy_client::person::Person;
use leptos::*;

#[component]
pub fn PersonView(cx: Scope, person: Person) -> impl IntoView {
    let Person {
        name,
        avatar,
        banned,
        local,
        updated,
        actor_id,
        admin,
        deleted,
        instance_id,
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
            <div class="font-lg flex flex-row">
                {name} {(!local)
                    .then(|| {
                        view! { cx, <div class="italic">"@" {actor_id.host_str().unwrap_or_default().to_string()}</div> }
                    })}
            </div>
        </a>
    }
}
