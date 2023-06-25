use std::sync::Arc;

use crate::error::Result;
use gloo_net::http::Request;
use post::{GetPosts, GetPostsResponse};
use serde_wasm_bindgen::to_value;
use url::{Url, quirks::host};
use log::info;
use serde::{Serialize, Deserialize, de::DeserializeOwned};
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
    let args = to_value(&HttpArgs { url: url.to_string() }).unwrap();
    let result = invoke("get_http", args).await;
    result.as_string()
}

async fn get_json<T>(url: &str) -> Option<T>
    where T: DeserializeOwned {
    let string_data = get_http(url).await?;
    serde_json::from_str(&string_data).ok()
}

impl CapyClient {
    pub fn new(hostname: impl ToString) -> Self {
        Self {
            inner: Arc::new(ClientImpl {
                hostname: hostname.to_string(),
                // client: Client::new(),
            }),
        }
    }

    pub async fn get_posts(&self, posts: GetPosts) -> Result<GetPostsResponse> {
        let query = serde_qs::to_string(&posts)?;
        let uri = &format!("{}/api/v3/post/list?{query}", self.inner.hostname);
        
        info!("fetching {uri}");
        // let text = self
        //     .inner
        //     .client
        //     .get(uri)
        //     .query(&posts)
        //     .fetch_mode_no_cors()
        //     .send()
        //     .await?
        //     .text()
        //     .await?;
        Ok(get_json(uri).await.unwrap())
    }
}


#[cfg(test)]
mod test {
    use crate::{CapyClient, post::GetPosts};
    fn test_client() -> CapyClient {
        CapyClient::new("https://lemmy.world".to_string())
    }

    #[tokio::test]
    async fn test_get_posts() {
        test_client().get_posts(GetPosts::default()).await.unwrap();
    }
}
