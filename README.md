wrapper for official openai api for chatgpt in Rust.

Currently support models `text-davinci-003` and `gpt-3.5-turbo`.

Todo:
[ ] context dialog 

use model  `text-davinci-003` 
```
use std::env;
use chatgpt_api::completions;

#[actix_rt::main]
async fn main() {
    let api_key = env::var("OPEN_AI_API_KEY").expect("OPEN_AI_API_KEY must be set");
let _ret=completions("how are you", &api_key).await.unwrap();

}
```
and `gpt-3.5-turbo`

```
use std::env;
use chatgpt_api::chat_completions;
    let api_key = env::var("OPEN_AI_API_KEY").expect("OPEN_AI_API_KEY must be set");
async fn gpt35_turbo() {
    let api_key = env::var("OPEN_AI_API_KEY").expect("OPEN_AI_API_KEY must be set");
    let _ret=chat_completions("how are you", &api_key).await;
}
```