use super::User;
use actix_identity::Identity;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct Session {
    pub user_id: i64,
    pub nickname: String,
}

impl Session {
    pub fn parse(id: &Identity) -> Option<Self> {
        match id.identity() {
            Some(string) => serde_json::from_str(&string).ok(),
            None => None,
        }
    }

    pub fn from_user(user: &User) -> Self {
        Session {
            user_id: user.id,
            nickname: user.nickname.clone(),
        }
    }
}
