//! mainly use model text-davinci-003
//! api: POST https://api.openai.com/v1/completions
use crate::error;
use openai_api_client::{
    ClientError, CompletionsParams, CompletionsResponse, ErrorResponse, Request,
};
use reqwest::{header, ClientBuilder};
use std::time::Duration;

static TEXT_DAVINCI_MODEL: &str = "text-davinci-003";
/// haedcode max-token and model inside
///
/// model: text-davinci-003
///
/// max_tokens = 3000(exclude prompt token)
pub async fn completions(prompt: &str, api_key: &str) -> std::result::Result<String, error::Errpr> {
    let model = TEXT_DAVINCI_MODEL;
    let max_tokens: u32 = 3000;
    let res = completions_inner(prompt, model, max_tokens, api_key).await?;
    Ok(res)
}
/// use reqwest crate to make http request
pub async fn completions_inner(
    prompt: &str,
    model: &str,
    max_tokens: u32,
    api_key: &str,
) -> std::result::Result<String, ClientError> {
    let params = CompletionsParams {
        model: model.to_string(),
        temperature: 0,
        max_tokens,
        top_p: 1.0,
        frequency_penalty: 0.0,
        presence_penalty: 0.0,
        stop: None,
        suffix: None,
        n: 1,
        stream: false,
        logprobs: None,
        echo: false,
        best_of: 1,
        logit_bias: None,
        user: None,
    };

    let request = Request {
        model: params.model.clone(),
        prompt: prompt.to_string(),
        temperature: params.temperature,
        max_tokens: params.max_tokens,
        top_p: params.top_p,
        frequency_penalty: params.frequency_penalty,
        presence_penalty: params.presence_penalty,
        stop: params.stop.clone(),
        suffix: params.suffix.clone(),
        logprobs: params.logprobs,
        echo: params.echo,
        best_of: params.best_of,
        n: params.n,
        stream: params.stream,
        logit_bias: params.logit_bias.clone(),
        user: params.user.clone(),
    };
    let request = serde_json::to_string(&request).unwrap();
    let mut header = header::HeaderMap::new();
    header.insert("Content-Type", "application/json".parse().unwrap());
    header.insert(
        "Authorization",
        format!("Bearer {api_key}").parse().unwrap(),
    );
    let client = ClientBuilder::new()
        .default_headers(header)
        .build()
        .unwrap();
    let response = client
        .post("https://api.openai.com/v1/completions")
        .timeout(Duration::from_secs(60))
        .body(request)
        .send()
        .await
        .map_err(|e| ClientError::NetworkError(format!("{e:?}")))?
        .bytes()
        .await
        .map_err(|e| ClientError::NetworkError(format!("{e:?}")))?;

    let response_str =
        std::str::from_utf8(&response).map_err(|e| ClientError::OtherError(format!("{e:?}")))?;

    let completions_response: CompletionsResponse = match serde_json::from_str(response_str) {
        Ok(response) => response,
        Err(e1) => {
            let error_response: ErrorResponse = match serde_json::from_str(response_str) {
                Ok(response) => response,
                Err(e2) => {
                    return Err(ClientError::OtherError(format!("{e2:?} {e1:?}")));
                }
            };
            return Err(ClientError::APIError(error_response.error.message));
        }
    };
    Ok(completions_response.choices[0].text.clone())
}
