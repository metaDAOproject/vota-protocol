use anchor_lang::prelude::Pubkey;
use serde::Deserialize;
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


pub fn serialize_pubkey_vec<S>(pubkeys: &Vec<Pubkey>, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
{
    let strings: Vec<String> = pubkeys.iter().map(|x| x.to_string()).collect();
    //add the []
    let result = format!("[{}]", &strings.join(","));
    serializer.serialize_str(&result.to_string())
}

pub fn deserialize_pubkey_vec<'de, D>(deserializer: D) -> Result<Vec<Pubkey>, D::Error>
    where
        D: serde::Deserializer<'de>,
{
    let s = <String as Deserialize>::deserialize(deserializer)?;
    //trim the [] from the string
    let s = s.trim_matches(|c| c == '[' || c == ']');
    Ok(s.split(",").map(|x| Pubkey::from_str(x).expect("Failed to deserialize pubkey")).collect())
}