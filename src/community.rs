use capybara_lemmy_client::{
    community::{CommunityId, GetCommunity},
    CapyClient,
};
use leptos::*;
use leptos_router::use_query_map;

#[derive(Debug, Clone, PartialEq, Eq)]
enum CommunityKey {
    Id(CommunityId),
    Url(String),
}

#[component]
pub fn Community(cx: Scope) -> impl IntoView {
    let query = use_query_map(cx);
    let community = create_memo(cx, move |_| {
        query.with(|q| {
            q.get("community").map(|s| {
                // the community can either be a community ID, or a string representing the url for the community
                match s.parse() {
                    Ok(val) => CommunityKey::Id(CommunityId(val)),
                    Err(e) => CommunityKey::Url(s.to_string()),
                }
            })
        })
    });
    let community = create_local_resource(cx, community, move |community| async move {
        let client = use_context::<CapyClient>(cx).unwrap();
        let (id, name) = match community {
            Some(CommunityKey::Id(id)) => (Some(id), None),
            Some(CommunityKey::Url(url)) => (None, Some(url)),
            None => (None, None),
        };
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
    }
}
