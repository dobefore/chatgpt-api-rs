use thiserror::Error;
#[derive(Error, Debug)]
pub enum Errpr {
    #[error("open_api_client client error: {0}")]
    Client(#[from] openai_api_client::ClientError),
    #[error("Json parsing error: {0}")]
    JsonParsing(#[from] serde_json::Error),

    #[error("reqwest error: {0}")]
    Reqwest(#[from] reqwest::Error),
}
