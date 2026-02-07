use std::collections::HashMap;
use tokio::sync::mpsc;

use crate::transport::messages::ServerMessage;

pub type Tx = mpsc::UnboundedSender<ServerMessage>;

#[derive(Default)]
pub struct ServerState {
    pub users: HashMap<String, Tx>,
}

impl ServerState {
    pub fn new() -> Self {
        Self {
            users: HashMap::new(),
        }
    }

    pub fn contains_user(&self, username: &str) -> bool {
        self.users.contains_key(username)
    }

    pub fn add_user(&mut self, username: String, tx: Tx) {
        self.users.insert(username, tx);
    }

    pub fn remove_user(&mut self, username: &str) {
        self.users.remove(username);
    }

    pub fn broadcast_message(&mut self, sender: &str, msg: &ServerMessage) {
        for (username, tx) in &self.users {
            if username != sender {
                // Ignore errors if a client disconnected
                let _ = tx.send(msg.clone());
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_server_state_user_management() {
        let mut state = ServerState::new();
        let (tx, _rx) = mpsc::unbounded_channel();

        // Test add_user
        state.add_user("Alice".to_string(), tx.clone());
        assert!(state.contains_user("Alice"));
        assert!(!state.contains_user("Bob"));

        // Test remove_user
        state.remove_user("Alice");
        assert!(!state.contains_user("Alice"));
    }

    #[test]
    fn test_broadcast() {
        // Need a multi-element broadcast test
        let mut state = ServerState::new();
        let (tx1, mut rx1) = mpsc::unbounded_channel();
        let (tx2, mut rx2) = mpsc::unbounded_channel();

        state.add_user("Alice".to_string(), tx1);
        state.add_user("Bob".to_string(), tx2);

        let msg = ServerMessage::Message {
            from: "Alice".to_string(),
            msg: "Hi".to_string(),
        };
        state.broadcast_message("Alice", &msg);

        // Bob should receive
        let received = rx2.try_recv();
        assert!(received.is_ok());

        // Alice should NOT receive (sender)
        let received_self = rx1.try_recv();
        assert!(received_self.is_err());
    }
}
