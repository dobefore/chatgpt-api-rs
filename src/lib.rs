pub mod chat_completion;
pub mod completion;
pub mod error;
pub use chat_completion::chat_completions;
pub use completion::completions;

// #[cfg(test)]
// mod tests {
//     use std::env;

//     use super::*;

//     #[actix_rt::test]
//    async fn it_works() {
//         // env::set_var ("https_proxy","socks5://127.0.0.1:10808");
//         // env::set_var ("HTTP_PROXY","http://127.0.0.1:10809");

//         let api_key = env::var("OPEN_AI_API_KEY").expect("OPEN_AI_API_KEY must be set");

//         let model = "text-davinci-003";
//         let max_tokens: u32 = 6;

// let r= completions_inner("introduce your self", model, max_tokens, &api_key).await.unwrap();
//     }
// }
