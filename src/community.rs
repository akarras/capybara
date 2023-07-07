use capybara_lemmy_client::{
    community::{CommunityId, GetCommunity},
    CapyClient,
};
use leptos::*;
use leptos_router::use_query_map;

use crate::components::posts::Posts;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum CommunityKey {
    Id(CommunityId),
    Name(String),
}

impl CommunityKey {
    pub fn id(&self) -> Option<CommunityId> {
        match self {
            CommunityKey::Id(id) => Some(*id),
            CommunityKey::Name(_) => None,
        }
    }

    pub fn name(&self) -> Option<String> {
        match self {
            CommunityKey::Id(_) => None,
            CommunityKey::Name(url) => Some(url.to_string()),
        }
    }
}

#[component]
pub fn Community(cx: Scope) -> impl IntoView {
    let query = use_query_map(cx);
    let community_id = create_memo(cx, move |_| {
        query.with(|q| {
            q.get("community").map(|s| {
                // the community can either be a community ID, or a string representing the url for the community
                match s.parse() {
                    Ok(val) => CommunityKey::Id(CommunityId(val)),
                    Err(_e) => CommunityKey::Name(s.to_string()),
                }
            })
        })
    });
    let community = create_local_resource(cx, community_id, move |community| async move {
        let client = use_context::<CapyClient>(cx).unwrap();
        let id = community.as_ref().and_then(|c| c.id());
        let name = community.as_ref().and_then(|c| c.name());
        let community = GetCommunity {
            id,
            name,
            ..Default::default()
        };
        client.execute(community).await.ok()
    });
    view! {cx,
    <Suspense fallback=move || "Loading">
        {move || {
            let community = community.read(cx);
            community.flatten().map(|c| view!{cx, <div></div>})
        }}
    </Suspense>
    <Posts community=Signal::Memo(community)/>
    }
}
