use capybara_lemmy_client::community::{Community, SubscribedType};
use leptos::*;
use leptos_use::{use_element_hover_with_options, UseElementHoverOptions};

use crate::components::{markdown::Markdown, subscribe::SubscribeButton, time::RelativeTime};

#[component]
pub fn CommunityBadge(
    cx: Scope,
    community: Community,
    subscribed: RwSignal<SubscribedType>,
) -> impl IntoView {
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
    let group_link = create_node_ref(cx);
    let popup = create_node_ref(cx);
    let hovered = use_element_hover_with_options(
        cx,
        group_link,
        UseElementHoverOptions::default()
            .delay_enter(500)
            .delay_leave(500),
    );
    let group_hover = use_element_hover_with_options(
        cx,
        popup,
        UseElementHoverOptions::default()
            .delay_enter(0)
            .delay_leave(500),
    );
    view! {cx,
        <div class="relative">
            <a href=format!("/c/{}", id.0) class="flex flex-row underline text-red-400 hover:text-red-600" class:font-bold=nsfw node_ref=group_link>
            {icon.as_ref().map(|icon| view!{cx, <img class="rounded w-6 h-6" src=icon.to_string()/>})}
            {name}
            {(!local).then(|| view!{cx, <div class="italic">"@"{actor_id.host_str().unwrap_or_default().to_string()}</div>})}

            </a>
            <div class="flex-col absolute top-10 left-10 p-5 z-10 w-96 h-96 overflow-y-auto bg-neutral-800 rounded-xl" class:hidden=move || { !(hovered() || group_hover()) } node_ref=popup>
                <div class="flex flex-col">
                    {banner.map(|b| view!{cx, <img src=b.to_string() class="h-fit w-96"/>})}
                    <div class="flex flex-row">
                        {icon.map(|icon| view!{cx, <img class="rounded w-12 h-12" src=icon.to_string()/>})}
                        <div class="flex flex-col">
                            <div class="flex flex-row">
                                <div class="text-lg">{title}</div>
                                {hidden.then(|| view!{cx, <div class="bg-gray-700 rounded p-1">"hidden"</div>})}
                                {removed.then(|| view!{cx, <div class="bg-gray-700 rounded p-1">"removed"</div>})}
                                {nsfw.then(|| view!{cx, <div class="bg-red-700 rounded p-1 text-white">"nsfw"</div>})}
                                {deleted.then(|| view!{cx, <div class="bg-neutral-600 rounded p-1">"deleted"</div>})}
                            </div>
                            <div class="flex flex-row gap-1">
                                <div class="text-gray-500">"created:"<RelativeTime time=published/></div>
                                {updated.map(|updated| view!{cx, <div class="text-gray-500">"(updated "<RelativeTime time=updated/>")"</div>})}
                            </div>
                        </div>
                    </div>
                    <SubscribeButton community_id=id subscribed />
                    {description.map(|description| view!{cx, <Markdown content=description />})}
                </div>
            </div>
        </div>
    }
}
