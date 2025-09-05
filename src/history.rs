#![allow(dead_code)]

use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct NotifHistory {
        #[serde(rename = "type")]
        pub notification_type: String,
        pub data: Vec<Vec<NotifItem>>,
}

#[derive(Debug, Deserialize)]
pub struct NotifItem {
        pub body: NotifData,
        pub message: NotifData,
        pub appname: NotifData,
        pub id: Id,
        pub timestamp: Timestamp,
        pub urgency: NotifData,
}

#[derive(Debug, Deserialize)]
pub struct NotifData {
        #[serde(rename = "type")]
        pub field_type: String,
        pub data: String,
}

#[derive(Debug, Deserialize)]
pub struct Id {
        #[serde(rename = "type")]
        pub field_type: String,
        pub data: u32,
}

#[derive(Debug, Deserialize)]
pub struct Timestamp {
        #[serde(rename = "type")]
        pub field_type: String,
        pub data: u64,
}

#[derive(Debug, Deserialize)]
pub struct Urgency {
        #[serde(rename = "type")]
        pub field_type: String,
        pub data: UrgencyLevel,
}

#[derive(Debug, Deserialize)]
pub enum UrgencyLevel {
        Low,
        Normal,
        Critical
}
