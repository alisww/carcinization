// // use crab::chron::{self, v2, v1};
// // use crab::*;
// // use futures_util::{pin_mut, StreamExt};
// // use serde_json::Value as JSONValue;
// //
// // #[tokio::main]
// // async fn main() {
// //     let c = reqwest::Client::new();
//     let req: v1::GameUpdatesRequest = v1::GameUpdatesRequestBuilder::default().game("939aaff2-e8bd-41f2-b733-ab393ba8047e").build().unwrap();
//     let s = chron::v1::fetch::<JSONValue, v1::GameUpdatesRequest>(&c, "https://api.sibr.dev/chronicler/v1/games/updates", req);
// //     pin_mut!(s);
// //
// //     while let Some(value) = s.next().await {
// //         println!("{}", serde_json::to_string_pretty(&value.unwrap()).unwrap());
// //     }
// // }
