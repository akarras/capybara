use thiserror::Error;

pub type Result<T> = core::result::Result<T, ClientError>;

#[derive(Error, Debug)]
pub enum ClientError {
    #[error("HTTP error {0}")]
    GlooNet(#[from] gloo_net::Error),
    #[error("Query string error {0}")]
    QueryString(#[from] serde_qs::Error),
    #[error("Json error {0}")]
    JsonError(#[from] serde_json::Error),
    #[error("HTTP Error")]
    HttpError,
    #[error("Must be authorized to use this API endpoint")]
    NotAuthorized,
}
