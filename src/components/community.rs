use capybara_lemmy_client::community::Community;
use leptos::*;

#[component]
pub fn CommunityBadge(cx: Scope, community: Community) -> impl IntoView {
    let Community {
        id,
        name,
        title,
        description,
        removed,
        published,
        updated,
        deleted,
        nsfw,
        actor_id,
        local,
        private_key,
        public_key,
        last_refreshed_at,
        icon,
        banner,
        followers_url,
        inbox_url,
        shared_inbox_url,
        hidden,
        posting_restricted_to_mods,
        instance_id,
        moderators_url,
        featured_url,
    } = community;
    view! {cx,
        <a href=format!("/community/{}", id.0) class="group flex flex-row underline text-red-400 hover:text-red-600">
            {icon.map(|icon| view!{cx, <img class="rounded w-6 h-6" src=icon.to_string()/>})}
            {name}
            {actor_id.to_string()}
            {(!local).then(|| view!{cx, <div class="">{actor_id.to_string()}</div>})}
            <div class="flex-col hidden group-hover:absolute p-5 z-10">
                <div>{title}</div>
                <div>{description}</div>
            </div>
            
        </a>
    }
}
