use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;
use strum::{Display, EnumString};
use url::Url;

use crate::{
    community::Community,
    instance::InstanceId,
    language::Language,
    local_user::LocalUserView,
    person::{Person, PersonView},
    post::{LanguageId, ListingType},
    sensitive::Sensitive,
};

#[derive(Debug, Serialize, Deserialize, Clone)]
/// A site view.
pub struct SiteView {
    pub site: Site,
    pub local_site: LocalSite,
    pub local_site_rate_limit: LocalSiteRateLimit,
    pub counts: SiteAggregates,
}

#[derive(PartialEq, Eq, Debug, Serialize, Deserialize, Clone)]
/// Aggregate data for a site.
pub struct SiteAggregates {
    pub id: i32,
    pub site_id: SiteId,
    pub users: i64,
    pub posts: i64,
    pub comments: i64,
    pub communities: i64,
    /// The number of users with any activity in the last day.
    pub users_active_day: i64,
    /// The number of users with any activity in the last week.
    pub users_active_week: i64,
    /// The number of users with any activity in the last month.
    pub users_active_month: i64,
    /// The number of users with any activity in the last half year.
    pub users_active_half_year: i64,
}

#[derive(Debug, Copy, Clone, Hash, Eq, PartialEq, Serialize, Deserialize, Default)]
/// The local site id.
pub struct LocalSiteId(i32);

#[skip_serializing_none]
#[derive(PartialEq, Eq, Debug, Clone, Serialize, Deserialize)]
/// Rate limits for your site. Given in count / length of time.
pub struct LocalSiteRateLimit {
    pub id: i32,
    pub local_site_id: LocalSiteId,
    pub message: i32,
    pub message_per_second: i32,
    pub post: i32,
    pub post_per_second: i32,
    pub register: i32,
    pub register_per_second: i32,
    pub image: i32,
    pub image_per_second: i32,
    pub comment: i32,
    pub comment_per_second: i32,
    pub search: i32,
    pub search_per_second: i32,
    pub published: chrono::NaiveDateTime,
    pub updated: Option<chrono::NaiveDateTime>,
}

#[skip_serializing_none]
#[derive(Debug, Serialize, Deserialize, Clone, Default)]
/// Fetches the site.
pub struct GetSite {
    pub auth: Option<Sensitive<String>>,
}

#[skip_serializing_none]
#[derive(Debug, Serialize, Deserialize, Clone)]
/// An expanded response for a site.
pub struct GetSiteResponse {
    pub site_view: SiteView,
    pub admins: Vec<PersonView>,
    pub version: String,
    pub my_user: Option<MyUserInfo>,
    pub all_languages: Vec<Language>,
    pub discussion_languages: Vec<LanguageId>,
    /// A list of taglines shown at the top of the front page.
    pub taglines: Vec<Tagline>,
    /// A list of custom emojis your site supports.
    pub custom_emojis: Vec<CustomEmojiView>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
/// A custom emoji view.
pub struct CustomEmojiView {
    pub custom_emoji: CustomEmoji,
    pub keywords: Vec<CustomEmojiKeyword>,
}

#[derive(Debug, Copy, Clone, Hash, Eq, PartialEq, Serialize, Deserialize, Default)]
/// The custom emoji id.
pub struct CustomEmojiId(i32);

#[skip_serializing_none]
#[derive(PartialEq, Eq, Debug, Clone, Serialize, Deserialize)]
/// A custom emoji.
pub struct CustomEmoji {
    pub id: CustomEmojiId,
    pub local_site_id: LocalSiteId,
    pub shortcode: String,
    pub image_url: DbUrl,
    pub alt_text: String,
    pub category: String,
    pub published: chrono::NaiveDateTime,
    pub updated: Option<chrono::NaiveDateTime>,
}

#[derive(PartialEq, Eq, Debug, Clone, Serialize, Deserialize)]
/// A custom keyword for an emoji.
pub struct CustomEmojiKeyword {
    pub id: i32,
    pub custom_emoji_id: CustomEmojiId,
    pub keyword: String,
}

#[skip_serializing_none]
#[derive(PartialEq, Eq, Debug, Clone, Serialize, Deserialize)]
/// A tagline, shown at the top of your site.
pub struct Tagline {
    pub id: i32,
    pub local_site_id: LocalSiteId,
    pub content: String,
    pub published: chrono::NaiveDateTime,
    pub updated: Option<chrono::NaiveDateTime>,
}

#[derive(Clone, PartialEq, Eq, Serialize, Deserialize, Debug)]
pub struct DbUrl(pub Url);

#[derive(Debug, Copy, Clone, Hash, Eq, PartialEq, Serialize, Deserialize, Default)]
/// The site id.
pub struct SiteId(i32);

#[skip_serializing_none]
#[derive(PartialEq, Eq, Debug, Clone, Serialize, Deserialize)]
/// The site.
pub struct Site {
    pub id: SiteId,
    pub name: String,
    /// A sidebar for the site in markdown.
    pub sidebar: Option<String>,
    pub published: chrono::NaiveDateTime,
    pub updated: Option<chrono::NaiveDateTime>,
    /// An icon URL.
    pub icon: Option<DbUrl>,
    /// A banner url.
    pub banner: Option<DbUrl>,
    /// A shorter, one-line description of the site.
    pub description: Option<String>,
    /// The federated actor_id.
    pub actor_id: DbUrl,
    /// The time the site was last refreshed.
    pub last_refreshed_at: chrono::NaiveDateTime,
    /// The site inbox
    pub inbox_url: DbUrl,
    pub private_key: Option<String>,
    pub public_key: String,
    pub instance_id: InstanceId,
}

#[skip_serializing_none]
#[derive(PartialEq, Eq, Debug, Clone, Serialize, Deserialize)]
/// The local site.
pub struct LocalSite {
    pub id: LocalSiteId,
    pub site_id: SiteId,
    /// True if the site is set up.
    pub site_setup: bool,
    /// Whether downvotes are enabled.
    pub enable_downvotes: bool,
    /// Whether NSFW is enabled.
    pub enable_nsfw: bool,
    /// Whether only admins can create communities.
    pub community_creation_admin_only: bool,
    /// Whether emails are required.
    pub require_email_verification: bool,
    /// An optional registration application questionnaire in markdown.
    pub application_question: Option<String>,
    /// Whether the instance is private or public.
    pub private_instance: bool,
    /// The default front-end theme.
    pub default_theme: String,
    pub default_post_listing_type: ListingType,
    /// An optional legal disclaimer page.
    pub legal_information: Option<String>,
    /// Whether to hide mod names on the modlog.
    pub hide_modlog_mod_names: bool,
    /// Whether new applications email admins.
    pub application_email_admins: bool,
    /// An optional regex to filter words.
    pub slur_filter_regex: Option<String>,
    /// The max actor name length.
    pub actor_name_max_length: i32,
    /// Whether federation is enabled.
    pub federation_enabled: bool,
    /// Whether captcha is enabled.
    pub captcha_enabled: bool,
    /// The captcha difficulty.
    pub captcha_difficulty: String,
    pub published: chrono::NaiveDateTime,
    pub updated: Option<chrono::NaiveDateTime>,
    pub registration_mode: RegistrationMode,
    /// Whether to email admins on new reports.
    pub reports_email_admins: bool,
}

#[derive(EnumString, Display, Debug, Serialize, Deserialize, Clone, Copy, PartialEq, Eq)]
/// The registration mode for your site. Determines what happens after a user signs up.
pub enum RegistrationMode {
    /// Closed to public.
    Closed,
    /// Open, but pending approval of a registration application.
    RequireApplication,
    /// Open to all.
    Open,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
/// Your user info.
pub struct MyUserInfo {
    pub local_user_view: LocalUserView,
    pub follows: Vec<CommunityFollowerView>,
    pub moderates: Vec<CommunityModeratorView>,
    pub community_blocks: Vec<CommunityBlockView>,
    pub person_blocks: Vec<PersonBlockView>,
    pub discussion_languages: Vec<LanguageId>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
/// A person block.
pub struct PersonBlockView {
    pub person: Person,
    pub target: Person,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
/// A community block.
pub struct CommunityBlockView {
    pub person: Person,
    pub community: Community,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
/// A community follower.
pub struct CommunityFollowerView {
    pub community: Community,
    pub follower: Person,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
/// A community moderator.
pub struct CommunityModeratorView {
    pub community: Community,
    pub moderator: Person,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
/// A community person ban.
pub struct CommunityPersonBanView {
    pub community: Community,
    pub person: Person,
}
