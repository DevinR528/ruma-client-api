//! De-/serialization functions for `Option<RoomEventFilter>` objects.
//! Delegates to `js_int::UInt` to ensure integer size is within bounds.

use serde::{
    de::{Error as _, Deserialize, Deserializer, DeserializeOwned},
    ser::{Error as _, Serialize, Serializer},
};
use serde_json;

/// Serialize a filter into a query string.
pub fn serialize<T, S>(filter: T, serializer: S) -> Result<S::Ok, S::Error>
where
    T: Serialize,
    S: Serializer,
{
    let json = serde_json::to_string(&filter).map_err(S::Error::custom)?;
    serializer.serialize_str(&json)
}

/// Deserializes a filter from a query string.
pub fn deserialize<'de, T, D>(deserializer: D) -> Result<T, D::Error>
where
    T: DeserializeOwned,
    D: Deserializer<'de>,
{
    let s = String::deserialize(deserializer)?;
    serde_json::from_str(&s).map_err(D::Error::custom)
}

#[cfg(test)]
mod tests {
    use std::convert::{TryFrom, TryInto};

    use serde::{Deserialize, Serialize};
    use serde_json::json;
    use ruma_identifiers::RoomId;

    use crate::r0::message::get_message_events::{Request, Direction};
    use crate::r0::filter::{LazyLoadOptions, RoomEventFilter};

    #[test]
    fn test_serialize_some_room_event_filter() {
        let room_id = RoomId::try_from("!roomid:example.org").unwrap();
        let filter = RoomEventFilter {
            lazy_load_options: LazyLoadOptions::Enabled { include_redundant_members: true, },
            rooms: Some(vec![ room_id.clone() ]),
            not_rooms: vec![ "room".into(), "room2".into(), "room3".into() ],
            not_types: vec![ "type".into() ],
            .. Default::default()
        };
        let req = Request {
            room_id,
            from: "token".into(),
            to: Some("token2".into()),
            dir: Direction::Backward,
            limit: Some(js_int::UInt::min_value()),
            filter: Some(filter),
        };

        let request: http::Request<Vec<u8>> = req.try_into().unwrap();
        println!("{:?}", request.uri().query());
    }

    #[test]
    fn test_serialize_none_room_event_filter() {
        let room_id = RoomId::try_from("!roomid:example.org").unwrap();
        let filter = RoomEventFilter {
            lazy_load_options: LazyLoadOptions::Enabled { include_redundant_members: true, },
            rooms: Some(vec![ room_id.clone() ]),
            not_rooms: vec![ "room".into() ],
            not_types: vec![ "type".into() ],
            .. Default::default()
        };
        let req = Request {
            room_id,
            from: "token".into(),
            to: Some("token2".into()),
            dir: Direction::Backward,
            limit: Some(js_int::UInt::min_value()),
            filter: None,
        };

        let request: http::Request<Vec<u8>> = req.try_into().unwrap();
        println!("{:?}", request.uri());
    }

    #[test]
    fn test_serialize_default_room_event_filter() {
        let room_id = RoomId::try_from("!roomid:example.org").unwrap();
        let req = Request {
            room_id,
            from: "token".into(),
            to: Some("token2".into()),
            dir: Direction::Backward,
            limit: Some(js_int::UInt::min_value()),
            filter: Some(RoomEventFilter::default()),
        };

        let request: http::Request<Vec<u8>> = req.try_into().unwrap();
        println!("{:?}", request.uri());
    }

}
