use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(tag = "type")]
pub enum ClientMessage {
    #[serde(rename = "join")]
    Join { username: String },

    #[serde(rename = "send")]
    Send { msg: String },

    #[serde(rename = "leave")]
    Leave,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(tag = "type")]
pub enum ServerMessage {
    #[serde(rename = "message")]
    Message { from: String, msg: String },

    #[serde(rename = "error")]
    Error { msg: String },
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_client_message_serialization() {
        let msg = ClientMessage::Join {
            username: "Alice".to_string(),
        };
        let json = serde_json::to_string(&msg).unwrap();
        assert_eq!(json, r#"{"type":"join","username":"Alice"}"#);

        let msg = ClientMessage::Send {
            msg: "Hello".to_string(),
        };
        let json = serde_json::to_string(&msg).unwrap();
        assert_eq!(json, r#"{"type":"send","msg":"Hello"}"#);

        let msg = ClientMessage::Leave;
        let json = serde_json::to_string(&msg).unwrap();
        assert_eq!(json, r#"{"type":"leave"}"#);
    }

    #[test]
    fn test_client_message_deserialization() {
        let json = r#"{"type":"join","username":"Bob"}"#;
        let msg: ClientMessage = serde_json::from_str(json).unwrap();
        match msg {
            ClientMessage::Join { username } => assert_eq!(username, "Bob"),
            _ => panic!("Expected Join"),
        }
    }
}
