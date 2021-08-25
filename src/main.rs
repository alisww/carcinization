// use crab::*;
// use crab::v2::*;
// use futures_util::pin_mut;
// use serde_json::Value as JSONValue;
//
// #[tokio::main]
// async fn main() {
//     let c = reqwest::Client::new();
//     let req: ChronV2Request = ChronV2RequestBuilder::default().ty("player").build().unwrap();
//     let s = v2::v2_fetch::<JSONValue>(&c, "https://api.sibr.dev/chronicler/v2/entities", req);
//     pin_mut!(s);
//
//     while let Some(value) = s.next().await {
//         println!("{:#?}",value);
//     }
// }
fn main() {}
