use super::{CarcinizationResult, PaginatedRequest, PaginatedResponse, SortOrder};
use chrono::{DateTime, Utc};
use derive_builder::Builder;
use futures_core::Stream;
use serde::{Deserialize, Serialize};
use std::collections::VecDeque;

/// A chronicler v2 request.
#[derive(Serialize, Builder, Debug, Default)]
#[builder(setter(into, strip_option), default)]
pub struct Request {
    #[serde(rename = "type")]
    ty: String,
    /// only supported when used with v2/entities
    #[serde(skip_serializing_if = "Option::is_none")]
    at: Option<DateTime<Utc>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub(crate) count: Option<usize>,
    #[serde(skip_serializing_if = "Option::is_none")]
    id: Option<String>,
    /// only supported when used with v2/versions
    #[serde(skip_serializing_if = "Option::is_none")]
    before: Option<DateTime<Utc>>,
    /// only supported when used with v2/versions
    #[serde(skip_serializing_if = "Option::is_none")]
    after: Option<DateTime<Utc>>,
    /// only supported when used with v2/versions
    #[serde(skip_serializing_if = "Option::is_none")]
    order: Option<SortOrder>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub(crate) page: Option<String>,
}

impl PaginatedRequest for Request {
    fn set_page(&mut self, page: &str) {
        self.page = Some(page.to_owned());
    }

    fn count(&self) -> Option<usize> {
        self.count
    }
}

#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct Response<T> {
    pub(crate) next_page: Option<String>,
    pub(crate) items: VecDeque<Entity<T>>,
}

#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct Entity<T> {
    pub entity_id: String,
    pub hash: String,
    pub valid_from: DateTime<Utc>,
    pub valid_to: Option<DateTime<Utc>>,
    pub data: T,
}

impl<T> PaginatedResponse<Entity<T>> for Response<T> {
    fn page(&self) -> Option<&str> {
        self.next_page.as_ref().map(|v| v.as_str())
    }

    fn data(&mut self) -> &mut VecDeque<Entity<T>> {
        &mut self.items
    }
}

pub fn fetch<'a, T: 'a + serde::de::DeserializeOwned>(
    c: &'a reqwest::Client,
    endpoint: &'a str,
    req: Request,
) -> impl Stream<Item = CarcinizationResult<Entity<T>>> + 'a {
    super::fetch::<Entity<T>, Response<T>, Request>(&c, endpoint, req)
}
