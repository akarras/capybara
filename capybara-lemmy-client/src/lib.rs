use std::{cell::RefCell, rc::Rc};

use crate::error::Result;
use async_trait::async_trait;
use comment::{
    CommentResponse, CreateComment, CreateCommentLike, GetComments, GetCommentsResponse,
    SaveComment,
};
use community::{
    CommunityResponse, FollowCommunity, GetCommunity, ListCommunities, ListCommunitiesResponse,
};
use error::ClientError;
use log::info;
use person::{GetPersonDetails, GetPersonDetailsResponse, Login, LoginResponse};
use post::{
    CreatePostLike, GetPost, GetPostResponse, GetPosts, GetPostsResponse, PostResponse, SavePost,
};
use sensitive::Sensitive;
use serde::{de::DeserializeOwned, Serialize};
use serde_wasm_bindgen::to_value;
use site::{GetSite, GetSiteResponse};
use wasm_bindgen::prelude::*;

pub use strum;

pub mod comment;
pub mod community;
pub mod error;
pub mod instance;
pub mod language;
pub mod local_user;
pub mod person;
/// This library is a rip from lemmy's own api_common.
pub mod post;
pub mod sensitive;
pub mod site;

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = ["window", "__TAURI__", "tauri"])]
    async fn invoke(cmd: &str, args: JsValue) -> JsValue;
}

#[derive(Clone)]
pub struct CapyClient {
    inner: Rc<RefCell<ClientImpl>>,
}

struct ClientImpl {
    hostname: String,
    jwt: Option<Sensitive<String>>,
}

#[derive(Serialize)]
struct HttpArgs {
    url: String,
}

#[derive(Serialize)]
struct HttpPostArgs {
    url: String,
    body: String,
}

async fn get_http(url: &str) -> Option<String> {
    let args = to_value(&HttpArgs {
        url: url.to_string(),
    })
    .unwrap();
    let result = invoke("get_http", args).await;
    result.as_string()
}

async fn get_json<T>(url: &str) -> Result<T>
where
    T: DeserializeOwned,
{
    let string_data = get_http(url).await.ok_or(ClientError::HttpError)?;
    info!("fetching {url}");
    // info!("returned json: {string_data}");
    Ok(serde_json::from_str(&string_data)?)
}

async fn post_http(url: &str, json_body: &impl Serialize) -> Option<String> {
    let body = serde_json::to_string(json_body).ok()?;
    let args = to_value(&HttpPostArgs {
        url: url.to_string(),
        body,
    })
    .unwrap();
    info!("fetching url {url}");
    let result = invoke("post_http", args).await;
    result.as_string()
}

async fn post_json<T, D>(url: &str, obj: &D) -> Result<T>
where
    T: DeserializeOwned,
    D: Serialize,
{
    info!("fetching url {url}");
    let string_data = post_http(url, obj).await.ok_or(ClientError::HttpError)?;
    info!("{string_data}");
    Ok(serde_json::from_str(&string_data)?)
}

pub enum HttpMode {
    GET,
    POST,
}

#[async_trait(?Send)]
pub trait LemmyRequest {
    type Response;
    fn get_path() -> &'static str;

    fn set_auth(&mut self, jwt: Option<Sensitive<String>>) -> Result<()>;

    fn get_http_mode() -> HttpMode;

    fn get_url(&self, client: &CapyClient) -> Result<String>
    where
        Self: Serialize,
    {
        let hostname = &client.inner.borrow().hostname;
        let path = Self::get_path();
        match Self::get_http_mode() {
            HttpMode::GET => {
                let query = serde_qs::to_string(&self)?;
                Ok(format!("{hostname}/api/v3{path}?{query}"))
            }
            HttpMode::POST => Ok(format!("{hostname}/api/v3{path}")),
        }
    }

    async fn execute(mut self, client: &CapyClient) -> Result<Self::Response>
    where
        Self::Response: DeserializeOwned + core::fmt::Debug,
        Self: Serialize,
        Self: Sized,
    {
        let auth = client.inner.borrow().jwt.clone();
        self.set_auth(auth)?;
        let url = self.get_url(client)?;
        match Self::get_http_mode() {
            HttpMode::GET => {
                let response = get_json(&url).await?;
                info!("GET {response:?}");
                return Ok(response);
            }
            HttpMode::POST => {
                let response = post_json(&url, &self).await?;
                info!("POST received {response:?}");
                return Ok(response);
            }
        }
    }
}

impl LemmyRequest for GetPost {
    type Response = GetPostResponse;

    fn get_path() -> &'static str {
        "/post"
    }

    fn set_auth(&mut self, jwt: Option<Sensitive<String>>) -> Result<()> {
        self.auth = jwt;
        Ok(())
    }

    fn get_http_mode() -> HttpMode {
        HttpMode::GET
    }
}

impl LemmyRequest for CreatePostLike {
    type Response = PostResponse;

    fn get_path() -> &'static str {
        "/post/like"
    }

    fn set_auth(&mut self, jwt: Option<Sensitive<String>>) -> Result<()> {
        self.auth = jwt.ok_or(ClientError::NotAuthorized)?;
        Ok(())
    }

    fn get_http_mode() -> HttpMode {
        HttpMode::POST
    }
}

impl LemmyRequest for GetPosts {
    type Response = GetPostsResponse;

    fn get_path() -> &'static str {
        "/post/list"
    }

    fn set_auth(&mut self, jwt: Option<Sensitive<String>>) -> Result<()> {
        self.auth = jwt;
        Ok(())
    }

    fn get_http_mode() -> HttpMode {
        HttpMode::GET
    }
}

impl LemmyRequest for GetComments {
    type Response = GetCommentsResponse;

    fn get_path() -> &'static str {
        "/comment/list"
    }

    fn set_auth(&mut self, jwt: Option<Sensitive<String>>) -> Result<()> {
        self.auth = jwt;
        Ok(())
    }

    fn get_http_mode() -> HttpMode {
        HttpMode::GET
    }
}

impl LemmyRequest for GetPersonDetails {
    type Response = GetPersonDetailsResponse;

    fn get_path() -> &'static str {
        "/user"
    }

    fn set_auth(&mut self, jwt: Option<Sensitive<String>>) -> Result<()> {
        self.auth = jwt;
        Ok(())
    }

    fn get_http_mode() -> HttpMode {
        HttpMode::GET
    }
}

impl LemmyRequest for Login {
    type Response = LoginResponse;

    fn get_path() -> &'static str {
        "/user/login"
    }

    fn set_auth(&mut self, _: Option<Sensitive<String>>) -> Result<()> {
        Ok(())
    }

    fn get_http_mode() -> HttpMode {
        HttpMode::POST
    }
}

impl LemmyRequest for GetSite {
    type Response = GetSiteResponse;

    fn get_path() -> &'static str {
        "/site"
    }

    fn set_auth(&mut self, jwt: Option<Sensitive<String>>) -> Result<()> {
        self.auth = jwt;
        Ok(())
    }

    fn get_http_mode() -> HttpMode {
        HttpMode::GET
    }
}

impl LemmyRequest for ListCommunities {
    type Response = ListCommunitiesResponse;

    fn get_path() -> &'static str {
        "/community/list"
    }

    fn set_auth(&mut self, jwt: Option<Sensitive<String>>) -> Result<()> {
        self.auth = jwt;
        Ok(())
    }

    fn get_http_mode() -> HttpMode {
        HttpMode::GET
    }
}

impl LemmyRequest for FollowCommunity {
    type Response = CommunityResponse;

    fn get_path() -> &'static str {
        "/community/follow"
    }

    fn set_auth(&mut self, jwt: Option<Sensitive<String>>) -> Result<()> {
        self.auth = jwt.ok_or(ClientError::NotAuthorized)?;
        Ok(())
    }

    fn get_http_mode() -> HttpMode {
        HttpMode::POST
    }
}

impl LemmyRequest for GetCommunity {
    type Response = CommunityResponse;

    fn get_path() -> &'static str {
        "/community"
    }

    fn set_auth(&mut self, jwt: Option<Sensitive<String>>) -> Result<()> {
        self.auth = jwt;
        Ok(())
    }

    fn get_http_mode() -> HttpMode {
        HttpMode::GET
    }
}

impl LemmyRequest for CreateCommentLike {
    type Response = CommentResponse;

    fn get_path() -> &'static str {
        "/comment/like"
    }

    fn set_auth(&mut self, jwt: Option<Sensitive<String>>) -> Result<()> {
        self.auth = jwt.ok_or(ClientError::NotAuthorized)?;
        Ok(())
    }

    fn get_http_mode() -> HttpMode {
        HttpMode::POST
    }
}

impl LemmyRequest for SaveComment {
    type Response = CommentResponse;

    fn get_path() -> &'static str {
        "/comment/save"
    }

    fn set_auth(&mut self, jwt: Option<Sensitive<String>>) -> Result<()> {
        self.auth = jwt.ok_or(ClientError::NotAuthorized)?;
        Ok(())
    }

    fn get_http_mode() -> HttpMode {
        HttpMode::POST
    }
}

impl LemmyRequest for SavePost {
    type Response = PostResponse;

    fn get_path() -> &'static str {
        "/post/save"
    }

    fn set_auth(&mut self, jwt: Option<Sensitive<String>>) -> Result<()> {
        self.auth = jwt.ok_or(ClientError::NotAuthorized)?;
        Ok(())
    }

    fn get_http_mode() -> HttpMode {
        HttpMode::POST
    }
}

impl LemmyRequest for CreateComment {
    type Response = CommentResponse;

    fn get_path() -> &'static str {
        "/comment"
    }

    fn set_auth(&mut self, jwt: Option<Sensitive<String>>) -> Result<()> {
        self.auth = jwt.ok_or(ClientError::NotAuthorized)?;
        Ok(())
    }

    fn get_http_mode() -> HttpMode {
        HttpMode::POST
    }
}

impl CapyClient {
    pub async fn execute<T>(&self, args: T) -> Result<T::Response>
    where
        T: LemmyRequest + Serialize + Sized,
        T::Response: DeserializeOwned + std::fmt::Debug,
    {
        args.execute(self).await
    }

    pub fn new(hostname: impl ToString, jwt: Option<Sensitive<String>>) -> Self {
        Self {
            inner: Rc::new(RefCell::new(ClientImpl {
                hostname: hostname.to_string(),
                jwt,
                // client: Client::new(),
            })),
        }
    }

    pub fn set_jwt(&self, jwt: Option<Sensitive<String>>) {
        self.inner.borrow_mut().jwt = jwt;
    }

    pub fn set_instance(&self, instance: String) {
        self.inner.borrow_mut().hostname = instance;
    }
}

// #[cfg(test)]
// mod test {
//     use crate::{post::GetPosts, CapyClient};
//     fn test_client() -> CapyClient {
//         CapyClient::new("https://lemmy.world".to_string())
//     }

//     #[tokio::test]
//     async fn test_get_posts() {
//         test_client().get_posts(GetPosts::default()).await.unwrap();
//     }
// }
