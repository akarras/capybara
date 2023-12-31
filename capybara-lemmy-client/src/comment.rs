use crate::{
    community::{Community, CommunityId, SubscribedType},
    local_user::LocalUserId,
    person::{Person, PersonId},
    post::{LanguageId, ListingType, Post, PostId},
    sensitive::Sensitive,
};
use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;
use strum::{Display, EnumIter, EnumString};
use url::Url;

#[derive(Debug, Copy, Clone, Hash, Eq, PartialEq, Default, Serialize, Deserialize)]
pub struct CommentId(pub i32);
#[skip_serializing_none]
#[derive(Debug, Serialize, Deserialize, Clone, Default)]
/// Get a list of comments.
pub struct GetComments {
    pub type_: Option<ListingType>,
    pub sort: Option<CommentSortType>,
    pub max_depth: Option<i32>,
    pub page: Option<i64>,
    pub limit: Option<i64>,
    pub community_id: Option<CommunityId>,
    pub community_name: Option<String>,
    pub post_id: Option<PostId>,
    pub parent_id: Option<CommentId>,
    pub saved_only: Option<bool>,
    pub auth: Option<Sensitive<String>>,
}

#[derive(
    EnumIter,
    EnumString,
    Display,
    Debug,
    Serialize,
    Deserialize,
    Clone,
    Copy,
    PartialEq,
    Eq,
    PartialOrd,
    Ord,
    Hash,
)]
/// The comment sort types. See here for descriptions: https://join-lemmy.org/docs/en/users/03-votes-and-ranking.html
pub enum CommentSortType {
    Hot,
    Top,
    New,
    Old,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
/// The comment list response.
pub struct GetCommentsResponse {
    pub comments: Vec<CommentView>,
}

#[skip_serializing_none]
#[derive(Debug, PartialEq, Eq, Serialize, Deserialize, Clone)]
/// A comment view.
pub struct CommentView {
    pub comment: Comment,
    pub creator: Person,
    pub post: Post,
    pub community: Community,
    pub counts: CommentAggregates,
    pub creator_banned_from_community: bool,
    pub subscribed: SubscribedType,
    pub saved: bool,
    pub creator_blocked: bool,
    pub my_vote: Option<i16>,
}

#[derive(Debug, Serialize, Deserialize, Clone, Default)]
/// Save / bookmark a comment.
pub struct SaveComment {
    pub comment_id: CommentId,
    pub save: bool,
    pub auth: Sensitive<String>,
}

#[skip_serializing_none]
#[derive(Clone, PartialEq, Eq, Debug, Serialize, Deserialize)]
/// A comment.
pub struct Comment {
    pub id: CommentId,
    pub creator_id: PersonId,
    pub post_id: PostId,
    pub content: String,
    /// Whether the comment has been removed.
    pub removed: bool,
    pub published: chrono::NaiveDateTime,
    pub updated: Option<chrono::NaiveDateTime>,
    /// Whether the comment has been deleted by its creator.
    pub deleted: bool,
    /// The federated activity id / ap_id.
    pub ap_id: Url,
    /// Whether the comment is local.
    pub local: bool,
    pub path: String,
    /// Whether the comment has been distinguished(speaking officially) by a mod.
    pub distinguished: bool,
    pub language_id: LanguageId,
}

#[derive(Debug, Serialize, Deserialize, Clone, Default)]
/// Like a comment.
pub struct CreateCommentLike {
    pub comment_id: CommentId,
    /// Must be -1, 0, or 1 .
    pub score: i16,
    pub auth: Sensitive<String>,
}

#[skip_serializing_none]
#[derive(Debug, Serialize, Deserialize, Clone)]
/// A comment response.
pub struct CommentResponse {
    pub comment_view: CommentView,
    pub recipient_ids: Vec<LocalUserId>,
    /// An optional front end ID, to tell which is coming back  
    pub form_id: Option<String>,
}

#[derive(PartialEq, Eq, Debug, Serialize, Deserialize, Clone)]
/// Aggregate data for a comment.
pub struct CommentAggregates {
    pub id: i32,
    pub comment_id: CommentId,
    pub score: i64,
    pub upvotes: i64,
    pub downvotes: i64,
    pub published: chrono::NaiveDateTime,
    /// The total number of children in this comment branch.
    pub child_count: i32,
    pub hot_rank: i32,
}

#[skip_serializing_none]
#[derive(Debug, Serialize, Deserialize, Clone, Default)]
/// Create a comment.
pub struct CreateComment {
    pub content: String,
    pub post_id: PostId,
    pub parent_id: Option<CommentId>,
    pub language_id: Option<LanguageId>,
    /// An optional front-end ID, to help UIs determine where the comment should go.
    pub form_id: Option<String>,
    pub auth: Sensitive<String>,
}
