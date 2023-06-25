use crate::{instance::InstanceId, person::Person};
use serde::{Deserialize, Serialize};
use strum::{Display, EnumString};
use url::Url;
use serde_with::skip_serializing_none;

#[derive(Debug, Copy, Clone, Hash, Eq, PartialEq, Default, Serialize, Deserialize)]
pub struct CommunityId(pub i32);

#[derive(EnumString, Display, Debug, PartialEq, Eq, Serialize, Deserialize, Clone, Copy)]
/// A type / status for a community subscribe.
pub enum SubscribedType {
    Subscribed,
    NotSubscribed,
    Pending,
}

#[derive(PartialEq, Eq, Debug, Serialize, Deserialize, Clone)]
/// Aggregate data for a community.
pub struct CommunityAggregates {
    pub id: i32,
    pub community_id: CommunityId,
    pub subscribers: i64,
    pub posts: i64,
    pub comments: i64,
    pub published: chrono::NaiveDateTime,
    /// The number of users with any activity in the last day.
    pub users_active_day: i64,
    /// The number of users with any activity in the last week.
    pub users_active_week: i64,
    /// The number of users with any activity in the last month.
    pub users_active_month: i64,
    /// The number of users with any activity in the last year.
    pub users_active_half_year: i64,
    pub hot_rank: i32,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
/// A community view.
pub struct CommunityView {
    pub community: Community,
    pub subscribed: SubscribedType,
    pub blocked: bool,
    pub counts: CommunityAggregates,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
/// A community moderator.
pub struct CommunityModeratorView {
    pub community: Community,
    pub moderator: Person,
}

#[skip_serializing_none]
#[derive(Debug, PartialEq, Eq, Serialize, Deserialize, Clone)]
/// A community.
pub struct Community {
    pub id: CommunityId,
    pub name: String,
    /// A longer title, that can contain other characters, and doesn't have to be unique.
    pub title: String,
    /// A sidebar / markdown description.
    pub description: Option<String>,
    /// Whether the community is removed by a mod.
    pub removed: bool,
    pub published: chrono::NaiveDateTime,
    pub updated: Option<chrono::NaiveDateTime>,
    /// Whether the community has been deleted by its creator.
    pub deleted: bool,
    /// Whether its an NSFW community.
    pub nsfw: bool,
    /// The federated actor_id.
    pub actor_id: Url,
    /// Whether the community is local.
    pub local: bool,
    #[serde(skip)]
    pub private_key: Option<String>,
    #[serde(skip)]
    pub public_key: String,
    #[serde(skip)]
    pub last_refreshed_at: chrono::NaiveDateTime,
    /// A URL for an icon.
    pub icon: Option<Url>,
    /// A URL for a banner.
    pub banner: Option<Url>,
    #[serde(skip_serializing)]
    pub followers_url: Option<Url>,
    #[serde(skip_serializing)]
    pub inbox_url: Option<Url>,
    #[serde(skip)]
    pub shared_inbox_url: Option<Url>,
    /// Whether the community is hidden.
    pub hidden: bool,
    /// Whether posting is restricted to mods only.
    pub posting_restricted_to_mods: bool,
    pub instance_id: InstanceId,
    /// Url where moderators collection is served over Activitypub
    #[serde(skip)]
    pub moderators_url: Option<Url>,
    /// Url where featured posts collection is served over Activitypub
    #[serde(skip)]
    pub featured_url: Option<Url>,
}
