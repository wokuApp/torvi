use mongodb::bson::{oid::ObjectId, DateTime};
use serde::{Serialize, Serializer};

pub fn serialize_oid<S: Serializer>(oid: &ObjectId, s: S) -> Result<S::Ok, S::Error> {
    s.serialize_str(&oid.to_hex())
}

pub fn serialize_option_oid<S: Serializer>(
    oid: &Option<ObjectId>,
    s: S,
) -> Result<S::Ok, S::Error> {
    match oid {
        Some(id) => s.serialize_str(&id.to_hex()),
        None => s.serialize_none(),
    }
}

pub fn serialize_vec_oid<S: Serializer>(
    oids: &[ObjectId],
    s: S,
) -> Result<S::Ok, S::Error> {
    let strings: Vec<String> = oids.iter().map(|id| id.to_hex()).collect();
    strings.serialize(s)
}

pub fn serialize_datetime<S: Serializer>(dt: &DateTime, s: S) -> Result<S::Ok, S::Error> {
    s.serialize_str(
        &dt.try_to_rfc3339_string()
            .unwrap_or_else(|_| dt.timestamp_millis().to_string()),
    )
}

pub fn serialize_option_datetime<S: Serializer>(
    dt: &Option<DateTime>,
    s: S,
) -> Result<S::Ok, S::Error> {
    match dt {
        Some(dt) => serialize_datetime(dt, s),
        None => s.serialize_none(),
    }
}
