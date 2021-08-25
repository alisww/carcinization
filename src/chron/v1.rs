use super::{CarcinizationResult, PaginatedRequest, PaginatedResponse, SortOrder};
use chrono::{DateTime, Utc};
use derive_builder::Builder;
use futures_core::Stream;
use serde::{Deserialize, Serialize};
use std::collections::VecDeque;

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Response<T> {
    next_page: Option<String>,
    data: VecDeque<T>,
}

impl<T> PaginatedResponse<T> for Response<T> {
    fn page(&self) -> Option<&str> {
        self.next_page.as_ref().map(|v| v.as_str())
    }

    fn data(&mut self) -> &mut VecDeque<T> {
        &mut self.data
    }
}

#[derive(Serialize, Builder, Debug, Default)]
#[builder(setter(into, strip_option), default)]
pub struct GameUpdatesRequest {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub(crate) count: Option<usize>,
    #[serde(skip_serializing_if = "Option::is_none")]
    game: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    before: Option<DateTime<Utc>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    after: Option<DateTime<Utc>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    order: Option<SortOrder>,
    #[serde(skip_serializing_if = "Option::is_none")]
    day: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    season: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    tournament: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    started: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    search: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub(crate) page: Option<String>,
}

impl PaginatedRequest for GameUpdatesRequest {
    fn set_page(&mut self, page: &str) {
        self.page = Some(page.to_owned());
    }

    fn count(&self) -> Option<usize> {
        self.count
    }
}

pub fn fetch<'a, T: 'a + serde::de::DeserializeOwned, R: 'a + PaginatedRequest + Serialize>(
    c: &'a reqwest::Client,
    endpoint: &'a str,
    req: R,
) -> impl Stream<Item = CarcinizationResult<T>> + 'a {
    super::fetch::<T, Response<T>, R>(&c, endpoint, req)
}
