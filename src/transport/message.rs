use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
#[serde(tag = "type")]
pub enum ClientMessage {
    #[serde(rename = "join")]
    Join {
        username: String,
    },

    #[serde(rename = "send")]
    Send {
        msg: String,
    },

    #[serde(rename = "leave")]
    Leave,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(tag = "type")]
pub enum ServerMessage {
    #[serde(rename = "message")]
    Message {
        from: String,
        msg: String,
    },

    #[serde(rename = "error")]
    Error {
        msg: String,
    },
}
