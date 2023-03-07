//! mainly use model text-gpt-3.5-turbo and gpt-3.5-turbo-0301
//! api: POST https://api.openai.com/v1/chat/completions
use openai_api_client::{ClientError, Usage};
use reqwest::{header, ClientBuilder};
use serde::{Deserialize, Serialize};
use std::{collections::HashMap, time::Duration};

use crate::error;
static URL: &str = "https://api.openai.com/v1/chat/completions";
static TEXT_GPT35_TURBO: &str = "gpt-3.5-turbo";
#[derive(Deserialize, Serialize)]
pub struct ChatCompletionsParams {
    pub model: String,
    /// \[
    ///    
    ///  {"role": "system", "content": "You are a helpful assistant."},
    ///
    ///     {"role": "user", "content": "Who won the world series in 2020?"},
    ///
    ///     {"role": "assistant", "content": "The Los Angeles Dodgers won the World Series in 2020."},
    ///   
    ///   {"role": "user", "content": "Where was it played?"}
    ///
    /// \]
    ///
    /// or simply \[ {"role": "user", "content": "Who won the world series in 2020?"}, \]
    pub messages: Vec<HashMap<String, String>>,
    pub temperature: u32,
    /// By default, the number of tokens the model can return will be (4096 - prompt tokens).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_tokens: Option<u32>,
    pub top_p: f32,
    pub frequency_penalty: f32,
    pub presence_penalty: f32,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub stop: Option<Vec<String>>,
    pub n: u32,
    pub stream: bool,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub logit_bias: Option<HashMap<String, i32>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user: Option<String>,
}

impl ChatCompletionsParams {
    pub fn new(messages: Vec<HashMap<String, String>>) -> Self {
        Self {
            messages,
            ..Default::default()
        }
    }
}

impl Default for ChatCompletionsParams {
    fn default() -> Self {
        ChatCompletionsParams {
            model: TEXT_GPT35_TURBO.to_string(),
            messages: Vec::new(),
            temperature: 1,
            top_p: 1.0,
            n: 1,
            stream: false,
            stop: None,
            max_tokens: None,
            frequency_penalty: 0.0,
            presence_penalty: 0.0,
            logit_bias: None,
            user: None,
        }
    }
}

#[derive(Deserialize, Serialize, Debug)]
pub struct ChatCompletionsResponse {
    pub id: String,
    pub object: String,
    pub created: u32,
    pub model: String,
    pub choices: Vec<ChatCompletionsChoice>,
    pub usage: Usage,
}
#[derive(Deserialize, Serialize, Debug)]
pub struct ChatCompletionsChoice {
    pub index: u32,
    pub message: HashMap<String, String>,
    pub finish_reason: String,
}
/// Use model `text-gpt-3.5-turbo`  to generate a chat completion.
///
/// this is a relatively easy wrapper for the api.
/// For the Prompt message,Assuming you are playing the role `user`,with no other roles invloved.
///
/// Note:Only return content,omiting role field
pub async fn chat_completions(
    prompt: &str,
    api_key: &str,
) -> std::result::Result<String, error::Errpr> {
    let mut msg = HashMap::new();
    msg.insert("role".to_string(), "user".to_string());
    msg.insert("content".to_string(), prompt.to_string());
    let req = ChatCompletionsParams::new(vec![msg]);
    let request = serde_json::to_string(&req)?;
    let mut header = header::HeaderMap::new();
    header.insert("Content-Type", "application/json".parse().unwrap());
    header.insert(
        "Authorization",
        format!("Bearer {api_key}").parse().unwrap(),
    );
    let client = ClientBuilder::new().default_headers(header).build()?;
    let response = client
        .post(URL)
        .timeout(Duration::from_secs(60))
        .body(request)
        .send()
        .await
        .map_err(|e| ClientError::NetworkError(format!("{e:?}")))?
        .bytes()
        .await
        .map_err(|e| ClientError::NetworkError(format!("{e:?}")))?;
    let mut completions_response: ChatCompletionsResponse = serde_json::from_slice(&response)?;
    Ok(completions_response.choices[0]
        .message
        .remove("content")
        .unwrap_or("".to_string()))
}
