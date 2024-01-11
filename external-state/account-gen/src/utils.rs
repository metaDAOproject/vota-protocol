use anchor_lang::prelude::Pubkey;
use serde::{Deserialize, Serializer};
use std::str::FromStr;

pub fn deserialize_pubkey<'de, D>(deserializer: D) -> Result<Pubkey, D::Error>
where
    D: serde::Deserializer<'de>,
{
    let s = <String as Deserialize>::deserialize(deserializer)?;
    Ok(Pubkey::from_str(&s).expect("Failed to deserialize pubkey"))
}

pub fn serialize_pubkey<S>(pubkey: &Pubkey, serializer: S) -> Result<S::Ok, S::Error>
where
    S: serde::Serializer,
{
    serializer.serialize_str(&pubkey.to_string())
}
