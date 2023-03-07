use std::env;

use chatgpt_api::{chat_completions, completions};
#[actix_rt::main]
async fn main() {
    let api_key = env::var("OPEN_AI_API_KEY").expect("OPEN_AI_API_KEY must be set");
    let _ret = completions("how are you", &api_key).await.unwrap();
    gpt35_turbo().await;
}

async fn gpt35_turbo() {
    let api_key = env::var("OPEN_AI_API_KEY").expect("OPEN_AI_API_KEY must be set");
    let _ret = chat_completions("how are you", &api_key).await;
}
