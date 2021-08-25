use chrono::{DateTime, Utc};
use derive_builder::Builder;
use serde::{Deserialize, Serialize};
use std::collections::VecDeque;

#[derive(Clone, Debug, Serialize)]
#[serde(rename_all = "lowercase")]
pub enum SortOrder {
    Asc,
    Desc,
}

#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct ChronV2Response<T> {
    pub(crate) next_page: Option<String>,
    pub(crate) items: VecDeque<ChronV2Entity<T>>,
}

#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct ChronV2Entity<T> {
    pub entity_id: String,
    pub hash: String,
    pub valid_from: DateTime<Utc>,
    pub valid_to: Option<DateTime<Utc>>,
    pub data: T,
}

/// A v2/entities request.
#[derive(Serialize, Builder, Debug, Default)]
#[builder(setter(into, strip_option), default)]
pub struct ChronV2Request {
    #[serde(rename = "type")]
    ty: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    at: Option<DateTime<Utc>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub(crate) count: Option<usize>,
    #[serde(skip_serializing_if = "Option::is_none")]
    id: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    before: Option<DateTime<Utc>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    after: Option<DateTime<Utc>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    order: Option<SortOrder>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub(crate) page: Option<String>,
}
