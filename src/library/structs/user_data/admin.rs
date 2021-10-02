use serde::{ Deserialize, Serialize };
use crate::library::structs::{ UserType, TrimmedStr };

#[derive(Deserialize, Serialize, Clone)]
pub struct AdminData {
    pub name: TrimmedStr,
    pub password: TrimmedStr
}

use std::convert::Into;
use crate::library::structs::User;
impl Into<User> for AdminData {
    fn into(self) -> User {
        User::new(self.name, UserType::Admin)
    }
}
