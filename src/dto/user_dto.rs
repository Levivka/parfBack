use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct UserDto {
    nickname: String,
    login: String,
    password: String,
}

impl UserDto {
    pub fn new(nickname: String, login: String, password: String) -> Self {
        Self {
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

    pub fn set_password(&mut self, password: String) {
        self.password = password;
    }

    pub fn password(&self) -> &String {
        &self.password
    }
}
