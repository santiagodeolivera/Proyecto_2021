use crate::library::program::memory::user::UserData;
use serde::{ Deserialize, Serialize };

#[derive(Deserialize, Serialize)]
pub struct JsonData {
    pub users: Vec<UserData>
}