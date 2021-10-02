use crate::library::structs::{ UserType, TrimmedStr };

#[derive(Debug)]
pub struct User {
    pub name: TrimmedStr,
    pub user_type: UserType
}

impl User {
    pub fn new(name: TrimmedStr, user_type: UserType) -> Self {
        User { name, user_type }
    }
}