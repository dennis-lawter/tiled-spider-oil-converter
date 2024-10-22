use serde::Deserialize;
use serde::Serialize;

#[derive(Debug, Serialize, Deserialize, PartialEq)]
pub enum RenderOrder {
    #[serde(rename = "right-down")]
    RightDown,
    #[serde(rename = "right-up")]
    RightUp,
    #[serde(rename = "left-down")]
    LeftDown,
    #[serde(rename = "left-up")]
    LeftUp,
}
