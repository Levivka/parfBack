use mongodb::bson::oid::ObjectId;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct User {
    id: ObjectId,
    nickname: String,
    login: String,
    password: String,
}
impl User {
    pub fn new(id: ObjectId, nickname: String, login: String, password: String) -> Self {
        Self {
            id,
            nickname,
            login,
            password,
        }
    }

    pub fn nickname(&self) -> &String {
        &self.nickname
    }

    pub fn set_nickname(&mut self, nickname: String) {
        self.nickname = nickname;
    }

    pub fn login(&self) -> &String {
        &self.login
    }

    pub fn set_login(&mut self, login: String) {
        self.login = login;
    }

    pub fn password(&self) -> &String {
        &self.password
    }

    pub fn set_password(&mut self, password: String) {
        self.password = password;
    }
    
    pub fn id(&self) -> &ObjectId {
        &self.id
    }
    
    pub fn set_id(&mut self, id: ObjectId) {
        self.id = id;
    }
}
