use anyhow::{Context, Result};
use serde::{Deserialize, Serialize};
#[derive(Deserialize, Serialize, Debug)]
pub enum PhaticMessage {
    Status(Event),
    Request(ListOptions),
    StatusList(EventList),
    TagRequest(TagListOptions),
    TagList(TagList),
}

#[derive(Deserialize, Serialize, Debug)]
pub struct Event {
    pub message: String,
    pub tags: Vec<String>,
    pub app: String,

    pub epoch_seconds: i64,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct EventList {
    pub events: Vec<Event>,
    pub last_id: i64,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct ListOptions {
    pub last_id: Option<i64>,
    pub limit: Option<i64>,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct TagListOptions {
    pub limit: Option<i64>,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct TagList {
    pub tags: Vec<String>,
}

pub fn serialize<T: Serialize>(value: &T) -> Result<String> {
    serde_json::to_string(value).context("when converting to json")
}

pub fn deserialize<'a, T: Deserialize<'a>>(value: &'a str) -> Result<T> {
    serde_json::from_str(value).context("when converting to json")
}
