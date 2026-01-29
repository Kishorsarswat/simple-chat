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
}
