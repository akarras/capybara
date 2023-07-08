use crate::{
    comment::CommentId,
    community::{Community, CommunityId, CommunityModeratorView, CommunityView, SubscribedType},
    person::{Person, PersonId},
    sensitive::Sensitive,
};
use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;
use strum::{Display, EnumString};
use url::Url;

#[derive(Debug, Copy, Clone, Hash, Eq, PartialEq, Default, Serialize, Deserialize)]
pub struct PostId(pub i32);

#[derive(Debug, Copy, Clone, Hash, Eq, PartialEq, Default, Serialize, Deserialize)]
pub struct LanguageId(pub i32);

#[skip_serializing_none]
#[derive(Debug, Serialize, Deserialize, Clone, Default)]
/// Create a post.
pub struct CreatePost {
    pub name: String,
    pub community_id: CommunityId,
    pub url: Option<Url>,
    /// An optional body for the post in markdown.
    pub body: Option<String>,
    /// A honeypot to catch bots. Should be None.
    pub honeypot: Option<String>,
    pub nsfw: Option<bool>,
    pub language_id: Option<LanguageId>,
    pub auth: Sensitive<String>,
}

#[skip_serializing_none]
#[derive(Clone, PartialEq, Eq, Debug, Serialize, Deserialize)]
/// A post.
pub struct Post {
    pub id: PostId,
    pub name: String,
    #[cfg_attr(feature = "full", ts(type = "string"))]
    /// An optional link / url for the post.
    pub url: Option<Url>,
    /// An optional post body, in markdown.
    pub body: Option<String>,
    pub creator_id: PersonId,
    pub community_id: CommunityId,
    /// Whether the post is removed.
    pub removed: bool,
    /// Whether the post is locked.
    pub locked: bool,
    pub published: chrono::NaiveDateTime,
    pub updated: Option<chrono::NaiveDateTime>,
    /// Whether the post is deleted.
    pub deleted: bool,
    /// Whether the post is NSFW.
    pub nsfw: bool,
    /// A title for the link.
    pub embed_title: Option<String>,
    /// A description for the link.
    pub embed_description: Option<String>,
    /// A thumbnail picture url.
    pub thumbnail_url: Option<Url>,
    /// The federated activity id / ap_id.
    pub ap_id: Url,
    /// Whether the post is local.
    pub local: bool,
    /// A video url for the link.
    pub embed_video_url: Option<Url>,
    pub language_id: LanguageId,
    /// Whether the post is featured to its community.
    pub featured_community: bool,
    /// Whether the post is featured to its site.
    pub featured_local: bool,
}

#[skip_serializing_none]
#[derive(Debug, PartialEq, Eq, Serialize, Deserialize, Clone)]
/// A post view.
pub struct PostView {
    pub post: Post,
    pub creator: Person,
    pub community: Community,
    pub creator_banned_from_community: bool,
    pub counts: PostAggregates,
    pub subscribed: SubscribedType,
    pub saved: bool,
    pub read: bool,
    pub creator_blocked: bool,
    pub my_vote: Option<i16>,
    pub unread_comments: i64,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct PostResponse {
    pub post_view: PostView,
}

#[skip_serializing_none]
#[derive(Debug, Serialize, Deserialize, Clone, Default)]
/// Get a post. Needs either the post id, or comment_id.
pub struct GetPost {
    pub id: Option<PostId>,
    pub comment_id: Option<CommentId>,
    pub auth: Option<Sensitive<String>>,
}

#[skip_serializing_none]
#[derive(Debug, Serialize, Deserialize, Clone)]
/// The post response.
pub struct GetPostResponse {
    pub post_view: PostView,
    pub community_view: CommunityView,
    pub moderators: Vec<CommunityModeratorView>,
    /// A list of cross-posts, or other times / communities this link has been posted to.
    pub cross_posts: Option<Vec<PostView>>,
}

#[derive(Hash, EnumString, Display, Debug, Serialize, Deserialize, Clone, Copy, PartialEq, Eq)]
pub enum SortType {
    Active,
    Hot,
    New,
    Old,
    TopDay,
    TopWeek,
    TopMonth,
    TopYear,
    TopAll,
    MostComments,
    NewComments,
    TopHour,
    TopSixHour,
    TopTwelveHour,
}

#[derive(Hash, EnumString, Display, Debug, Serialize, Deserialize, Clone, Copy, PartialEq, Eq)]
/// A listing type for post and comment list fetches.
pub enum ListingType {
    /// Content from your own site, as well as all connected / federated sites.
    All,
    /// Content from your site only.
    Local,
    /// Content only from communities you've subscribed to.
    Subscribed,
}

#[skip_serializing_none]
#[derive(Serialize, Deserialize, Debug, Clone, Default)]
/// Get a list of posts.
pub struct GetPosts {
    pub type_: Option<ListingType>,
    pub sort: Option<SortType>,
    pub page: Option<i64>,
    pub limit: Option<i64>,
    pub community_id: Option<CommunityId>,
    pub community_name: Option<String>,
    pub saved_only: Option<bool>,
    pub auth: Option<Sensitive<String>>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
/// The post list response.
pub struct GetPostsResponse {
    pub posts: Vec<PostView>,
}

#[derive(Debug, Serialize, Deserialize, Clone, Default)]
/// Like a post.
pub struct CreatePostLike {
    pub post_id: PostId,
    /// Score must be -1, 0, or 1.
    pub score: i16,
    pub auth: Sensitive<String>,
}

#[skip_serializing_none]
#[derive(Debug, Serialize, Deserialize, Clone, Default)]
/// Edit a post.
pub struct EditPost {
    pub post_id: PostId,
    pub name: Option<String>,
    pub url: Option<Url>,
    /// An optional body for the post in markdown.
    pub body: Option<String>,
    pub nsfw: Option<bool>,
    pub language_id: Option<LanguageId>,
    pub auth: Sensitive<String>,
}

#[derive(Debug, Serialize, Deserialize, Clone, Default)]
/// Delete a post.
pub struct DeletePost {
    pub post_id: PostId,
    pub deleted: bool,
    pub auth: Sensitive<String>,
}

#[skip_serializing_none]
#[derive(Debug, Serialize, Deserialize, Clone, Default)]
/// Remove a post (only doable by mods).
pub struct RemovePost {
    pub post_id: PostId,
    pub removed: bool,
    pub reason: Option<String>,
    pub auth: Sensitive<String>,
}

#[derive(Debug, Serialize, Deserialize, Clone, Default)]
/// Mark a post as read.
pub struct MarkPostAsRead {
    pub post_id: PostId,
    pub read: bool,
    pub auth: Sensitive<String>,
}

#[derive(Debug, Serialize, Deserialize, Clone, Default)]
/// Lock a post (prevent new comments).
pub struct LockPost {
    pub post_id: PostId,
    pub locked: bool,
    pub auth: Sensitive<String>,
}

#[derive(
    EnumString, Display, Debug, Serialize, Deserialize, Clone, Copy, Default, PartialEq, Eq,
)]
/// The feature type for a post.
pub enum PostFeatureType {
    #[default]
    /// Features to the top of your site.
    Local,
    /// Features to the top of the community.
    Community,
}

#[derive(Debug, Serialize, Deserialize, Clone, Default)]
/// Feature a post (stickies / pins to the top).
pub struct FeaturePost {
    pub post_id: PostId,
    pub featured: bool,
    pub feature_type: PostFeatureType,
    pub auth: Sensitive<String>,
}

#[derive(Debug, Serialize, Deserialize, Clone, Default)]
/// Save / bookmark a post.
pub struct SavePost {
    pub post_id: PostId,
    pub save: bool,
    pub auth: Sensitive<String>,
}

#[derive(Debug, Serialize, Deserialize, Clone, Default)]
/// Create a post report.
pub struct CreatePostReport {
    pub post_id: PostId,
    pub reason: String,
    pub auth: Sensitive<String>,
}

#[derive(Debug, Copy, Clone, Hash, Eq, PartialEq, Serialize, Deserialize, Default)]
pub struct PostReportId(pub i32);

#[skip_serializing_none]
#[derive(PartialEq, Eq, Serialize, Deserialize, Debug, Clone)]
/// A post report.
pub struct PostReport {
    pub id: PostReportId,
    pub creator_id: PersonId,
    pub post_id: PostId,
    /// The original post title.
    pub original_post_name: String,
    /// The original post url.
    pub original_post_url: Option<Url>,
    /// The original post body.
    pub original_post_body: Option<String>,
    pub reason: String,
    pub resolved: bool,
    pub resolver_id: Option<PersonId>,
    pub published: chrono::NaiveDateTime,
    pub updated: Option<chrono::NaiveDateTime>,
}

#[skip_serializing_none]
#[derive(Debug, PartialEq, Eq, Serialize, Deserialize, Clone)]
/// A post report view.
pub struct PostReportView {
    pub post_report: PostReport,
    pub post: Post,
    pub community: Community,
    pub creator: Person,
    pub post_creator: Person,
    pub creator_banned_from_community: bool,
    pub my_vote: Option<i16>,
    pub counts: PostAggregates,
    pub resolver: Option<Person>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
/// The post report response.
pub struct PostReportResponse {
    pub post_report_view: PostReportView,
}

#[derive(Debug, Serialize, Deserialize, Clone, Default)]
/// Resolve a post report (mods only).
pub struct ResolvePostReport {
    pub report_id: PostReportId,
    pub resolved: bool,
    pub auth: Sensitive<String>,
}

#[skip_serializing_none]
#[derive(Debug, Serialize, Deserialize, Clone, Default)]
/// List post reports.
pub struct ListPostReports {
    pub page: Option<i64>,
    pub limit: Option<i64>,
    /// Only shows the unresolved reports
    pub unresolved_only: Option<bool>,
    /// if no community is given, it returns reports for all communities moderated by the auth user
    pub community_id: Option<CommunityId>,
    pub auth: Sensitive<String>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
/// The post reports response.
pub struct ListPostReportsResponse {
    pub post_reports: Vec<PostReportView>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
/// Get metadata for a given site.
pub struct GetSiteMetadata {
    pub url: Url,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
/// The site metadata response.
pub struct GetSiteMetadataResponse {
    pub metadata: SiteMetadata,
}

#[skip_serializing_none]
#[derive(Debug, Deserialize, Serialize, PartialEq, Eq, Clone)]
/// Site metadata, from its opengraph tags.
pub struct SiteMetadata {
    pub title: Option<String>,
    pub description: Option<String>,
    pub(crate) image: Option<Url>,
    pub embed_video_url: Option<Url>,
}

#[derive(PartialEq, Eq, Debug, Serialize, Deserialize, Clone)]
/// Aggregate data for a post.
pub struct PostAggregates {
    pub id: i32,
    pub post_id: PostId,
    pub comments: i64,
    pub score: i64,
    pub upvotes: i64,
    pub downvotes: i64,
    pub published: chrono::NaiveDateTime,
    /// A newest comment time, limited to 2 days, to prevent necrobumping  
    pub newest_comment_time_necro: chrono::NaiveDateTime,
    /// The time of the newest comment in the post.
    pub newest_comment_time: chrono::NaiveDateTime,
    /// If the post is featured on the community.
    pub featured_community: bool,
    /// If the post is featured on the site / to local.
    pub featured_local: bool,
    pub hot_rank: i32,
    pub hot_rank_active: i32,
}
