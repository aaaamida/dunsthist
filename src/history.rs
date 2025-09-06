#![allow(dead_code)]

use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct NotifHistory {
        #[serde(flatten, deserialize_with = "flatten_data")]
        pub data: Vec<NotifItem>,
}

fn flatten_data<'d, D: serde::Deserializer<'d>>(des: D) -> Result<Vec<NotifItem>, D::Error> {
        #[derive(Deserialize)]
        struct Wrapper {
                data: Vec<Vec<NotifItem>>
        }

        let wrapper = Wrapper::deserialize(des)?;
        Ok(wrapper.data.into_iter().flatten().collect())
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
        pub data: String,
}

#[derive(Debug, Deserialize)]
pub struct Id {
        pub data: u32,
}

#[derive(Debug, Deserialize)]
pub struct Timestamp {
        pub data: u64,
}
