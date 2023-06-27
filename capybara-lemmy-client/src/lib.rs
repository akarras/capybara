use std::sync::Arc;

use crate::error::Result;
use async_trait::async_trait;
use error::ClientError;
use gloo_net::http::Request;
use log::info;
use post::{GetPost, GetPostResponse, GetPosts, GetPostsResponse, PostId, PostResponse};
use serde::{de::DeserializeOwned, Deserialize, Serialize};
use serde_wasm_bindgen::to_value;
use url::{quirks::host, Url};
use wasm_bindgen::prelude::*;

pub mod comment;
pub mod community;
pub mod error;
pub mod instance;
pub mod person;
/// This library is a rip from lemmy's own api_common.
pub mod post;
pub mod sensitive;

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = ["window", "__TAURI__", "tauri"])]
    async fn invoke(cmd: &str, args: JsValue) -> JsValue;
}

#[derive(Clone)]
pub struct CapyClient {
    inner: Arc<ClientImpl>,
}

struct ClientImpl {
    hostname: String,
    // client: Client,
}

#[derive(Deserialize, Serialize)]
struct HttpArgs {
    url: String,
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
    Ok(serde_json::from_str(&string_data)?)
}

#[async_trait(?Send)]
pub trait LemmyRequest {
    type Response;
    fn get_path() -> &'static str;
    async fn execute(&self, client: &CapyClient) -> Result<Self::Response>
    where
        Self::Response: DeserializeOwned,
        Self: Serialize,
    {
        let hostname = &client.inner.hostname;
        let path = Self::get_path();
        let query = serde_qs::to_string(&self)?;
        let url = format!("{hostname}/api/v3{path}?{query}");
        info!("fetching {url}");
        let response = get_json(&url).await?;
        Ok(response)
    }
}

impl LemmyRequest for GetPost {
    type Response = GetPostResponse;

    fn get_path() -> &'static str {
        "/post"
    }
}

impl LemmyRequest for GetPosts {
    type Response = GetPostsResponse;

    fn get_path() -> &'static str {
        "/post/list"
    }
}

impl CapyClient {
    pub async fn execute<T>(&self, args: T) -> Result<T::Response>
    where
        T: LemmyRequest + Serialize,
        T::Response: DeserializeOwned,
    {
        args.execute(self).await
    }

    pub fn new(hostname: impl ToString) -> Self {
        Self {
            inner: Arc::new(ClientImpl {
                hostname: hostname.to_string(),
                // client: Client::new(),
            }),
        }
    }

    pub async fn get_posts(&self, posts: GetPosts) -> Result<GetPostsResponse> {
        self.execute(posts).await
    }

    pub async fn get_post(&self, get_post: GetPost) -> Result<GetPostResponse> {
        self.execute(get_post).await
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
