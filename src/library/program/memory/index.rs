use crate::library::program::memory::user::UserList;
use serde::{ Deserialize, Serialize };

#[derive(Deserialize, Serialize, Clone)]
pub struct JsonData {
    pub users: UserList
}
