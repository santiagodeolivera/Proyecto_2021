use serde::{ Deserialize, Serialize };
use crate::library::memory::UserType;

#[derive(Deserialize, Serialize)]
pub struct UserData {
    pub name: String,
    pub password: String,
    pub user_type: UserType
}

use std::convert::Into;
use crate::library::memory::User;
impl Into<User> for UserData {
    fn into(self) -> User {
        User::new(self.name, self.user_type)
    }
}