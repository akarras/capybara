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
        <a class="flex flex-row underline text-red-400" class:bold=admin class:italic=local class:line-through=deleted href=format!("/person/{}", id.0)>
            {avatar
                .map(|a| {
                    view! { cx, <img class="w-10 h-10 rounded-full p-r-2" src=a.to_string()/> }
                })}
            <div class="font-lg">{name}</div>
        </a>
    }
}
