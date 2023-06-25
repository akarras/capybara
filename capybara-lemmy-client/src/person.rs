use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;
use url::Url;

use crate::instance::InstanceId;

#[derive(Debug, Copy, Clone, Hash, Eq, PartialEq, Default, Serialize, Deserialize)]
pub struct PersonId(pub i32);

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
    #[serde(skip_serializing)]
    pub inbox_url: Url,
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
