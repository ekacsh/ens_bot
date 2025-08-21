use serde::{Deserialize, Deserializer};

pub fn deserialize_discord_id<'de, D>(deserializer: D) -> Result<u64, D::Error>
where
    D: Deserializer<'de>,
{
    let buf = String::deserialize(deserializer)?;

    buf.parse::<u64>().map_err(serde::de::Error::custom)
}
