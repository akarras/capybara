use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;
use url::Url;

use crate::{
    comment::CommentView,
    community::{CommunityId, CommunityModeratorView},
    instance::InstanceId,
    post::{PostView, SortType},
    sensitive::Sensitive,
};

#[derive(Debug, Copy, Clone, Hash, Eq, PartialEq, Default, Serialize, Deserialize)]
pub struct PersonId(pub i32);

#[skip_serializing_none]
#[derive(Debug, Serialize, Deserialize, Clone, Default)]
/// Logging into lemmy.
pub struct Login {
    pub username_or_email: Sensitive<String>,
    pub password: Sensitive<String>,
    /// May be required, if totp is enabled for their account.
    pub totp_2fa_token: Option<String>,
}

#[skip_serializing_none]
#[derive(Debug, Serialize, Deserialize, Clone)]
/// A response for your login.
pub struct LoginResponse {
    /// This is None in response to `Register` if email verification is enabled, or the server requires registration applications.
    pub jwt: Option<Sensitive<String>>,
    /// If registration applications are required, this will return true for a signup response.
    pub registration_created: bool,
    /// If email verifications are required, this will return true for a signup response.
    pub verify_email_sent: bool,
}

#[skip_serializing_none]
#[derive(Clone, PartialEq, Eq, Debug, Serialize, Deserialize)]
pub struct Person {
    pub id: PersonId,
    pub name: String,
    /// A shorter display name.
    pub display_name: Option<String>,
    /// A URL for an avatar.
    pub avatar: Option<Url>,
    /// Whether the person is banned.
    pub banned: bool,
    pub published: chrono::NaiveDateTime,
    pub updated: Option<chrono::NaiveDateTime>,
    /// The federated actor_id.
    pub actor_id: Url,
    /// An optional bio, in markdown.
    pub bio: Option<String>,
    /// Whether the person is local to our site.
    pub local: bool,
    #[serde(skip)]
    pub private_key: Option<String>,
    #[serde(skip)]
    pub public_key: String,
    #[serde(skip)]
    pub last_refreshed_at: chrono::NaiveDateTime,
    /// A URL for a banner.
    pub banner: Option<Url>,
    /// Whether the person is deleted.
    pub deleted: bool,
    #[serde(skip)]
    pub inbox_url: Option<Url>,
    #[serde(skip)]
    pub shared_inbox_url: Option<Url>,
    /// A matrix id, usually given an @person:matrix.org
    pub matrix_user_id: Option<String>,
    /// Whether the person is an admin.
    pub admin: bool,
    /// Whether the person is a bot account.
    pub bot_account: bool,
    /// When their ban, if it exists, expires, if at all.
    pub ban_expires: Option<chrono::NaiveDateTime>,
    pub instance_id: InstanceId,
}

#[skip_serializing_none]
#[derive(Debug, Serialize, Deserialize, Clone, Default)]
pub struct GetPersonDetails {
    pub person_id: Option<PersonId>,
    /// Example: dessalines , or dessalines@xyz.tld
    pub username: Option<String>,
    pub sort: Option<SortType>,
    pub page: Option<i64>,
    pub limit: Option<i64>,
    pub community_id: Option<CommunityId>,
    pub saved_only: Option<bool>,
    pub auth: Option<Sensitive<String>>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
/// A person's details response.
pub struct GetPersonDetailsResponse {
    pub person_view: PersonView,
    pub comments: Vec<CommentView>,
    pub posts: Vec<PostView>,
    pub moderates: Vec<CommunityModeratorView>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
/// A person view.
pub struct PersonView {
    pub person: Person,
    pub counts: PersonAggregates,
}

#[derive(PartialEq, Eq, Debug, Serialize, Deserialize, Clone, Default)]
/// Aggregate data for a person.
pub struct PersonAggregates {
    pub id: i32,
    pub person_id: PersonId,
    pub post_count: i64,
    pub post_score: i64,
    pub comment_count: i64,
    pub comment_score: i64,
}
