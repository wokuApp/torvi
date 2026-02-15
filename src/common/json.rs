use mongodb::bson::oid::ObjectId;
use mongodb::bson::DateTime;
use serde::{de, Deserialize, Deserializer, Serialize, Serializer};

/// Deserialize an ObjectId from a hex string (e.g. "507f1f77bcf86cd799439011")
pub fn deserialize_oid<'de, D: Deserializer<'de>>(d: D) -> Result<ObjectId, D::Error> {
    let s = String::deserialize(d)?;
    ObjectId::parse_str(&s).map_err(de::Error::custom)
}

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
