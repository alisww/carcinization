use super::*;
use async_stream::try_stream;
use futures_core::stream::Stream;
use std::collections::VecDeque;

pub fn v2_fetch<'a, T: 'a + serde::de::DeserializeOwned>(
    client: &'a reqwest::Client,
    endpoint: &'a str,
    mut req: ChronV2Request,
) -> impl Stream<Item = CarcinizationResult<ChronV2Entity<T>>> + 'a {
    try_stream! {
        let mut buffer: VecDeque<ChronV2Entity<T>> = VecDeque::with_capacity(req.count.unwrap_or(100));

        loop {
            let mut chron_response: ChronV2Response<T> = client
                .get(endpoint)
                .query(&req)
                .send()
                .await?
                .json()
                .await?;
            buffer.append(&mut chron_response.items);

            while let Some(item) = buffer.pop_front() {
                yield item;
            }

            if let Some(next_page) = chron_response.next_page {
                req.page = Some(next_page);
            } else {
                break;
            }
        }
    }
}
