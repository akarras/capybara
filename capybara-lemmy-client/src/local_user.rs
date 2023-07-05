use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

use crate::{
    person::{Person, PersonAggregates, PersonId},
    post::{ListingType, SortType},
};

#[derive(Debug, Serialize, Deserialize, Clone)]
/// A local user view.
pub struct LocalUserView {
    pub local_user: LocalUser,
    pub person: Person,
    pub counts: PersonAggregates,
}

#[derive(Debug, Copy, Clone, Hash, Eq, PartialEq, Default, Serialize, Deserialize)]
/// The local user id.
pub struct LocalUserId(pub i32);

#[skip_serializing_none]
#[derive(Clone, PartialEq, Eq, Debug, Serialize, Deserialize)]
/// A local user.
pub struct LocalUser {
    pub id: LocalUserId,
    /// The person_id for the local user.
    pub person_id: PersonId,
    #[serde(skip)]
    pub password_encrypted: String,
    pub email: Option<String>,
    /// Whether to show NSFW content.
    pub show_nsfw: bool,
    pub theme: String,
    pub default_sort_type: SortType,
    pub default_listing_type: ListingType,
    pub interface_language: String,
    /// Whether to show avatars.
    pub show_avatars: bool,
    pub send_notifications_to_email: bool,
    /// A validation ID used in logging out sessions.
    pub validator_time: chrono::NaiveDateTime,
    /// Whether to show comment / post scores.
    pub show_scores: bool,
    /// Whether to show bot accounts.
    pub show_bot_accounts: bool,
    /// Whether to show read posts.
    pub show_read_posts: bool,
    /// Whether to show new posts as notifications.
    pub show_new_post_notifs: bool,
    /// Whether their email has been verified.
    pub email_verified: bool,
    /// Whether their registration application has been accepted.
    pub accepted_application: bool,
    #[serde(skip)]
    pub totp_2fa_secret: Option<String>,
    /// A URL to add their 2-factor auth.
    pub totp_2fa_url: Option<String>,
    /// Open links in a new tab.
    pub open_links_in_new_tab: Option<bool>,
}
