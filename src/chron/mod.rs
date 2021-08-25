use super::*;
use async_stream::try_stream;
use futures_core::stream::Stream;
use serde::{de::DeserializeOwned, Serialize};
use std::collections::VecDeque;

pub mod v1;
pub mod v2;

pub trait PaginatedRequest {
    fn set_page(&mut self, page: &str) -> ();
    fn count(&self) -> Option<usize>;
}

pub trait PaginatedResponse<T> {
    fn page(&self) -> Option<&str>;
    fn data(&mut self) -> &mut VecDeque<T>;
}

pub fn fetch<'a, T: 'a, R: 'a, Q: 'a>(
    client: &'a reqwest::Client,
    endpoint: &'a str,
    mut req: Q,
) -> impl Stream<Item = CarcinizationResult<T>> + 'a
where
    T: DeserializeOwned,
    R: PaginatedResponse<T> + DeserializeOwned,
    Q: PaginatedRequest + Serialize,
{
    try_stream! {
        let mut buffer: VecDeque<T> = VecDeque::with_capacity(req.count().unwrap_or(100));

        loop {
            let mut chron_response: R = client
                .get(endpoint)
                .query(&req)
                .send()
                .await?
                .json()
                .await?;
            buffer.append(&mut chron_response.data());

            while let Some(item) = buffer.pop_front() {
                yield item;
            }

            if let Some(next_page) = chron_response.page() {
                req.set_page(next_page);
            } else {
                break;
            }
        }
    }
}
