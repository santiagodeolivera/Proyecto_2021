use crate::library::memory::UserType;

#[derive(Debug)]
pub struct User {
    pub name: String,
    pub user_type: UserType
}

impl User {
    pub fn new(name: String, user_type: UserType) -> Self {
        User { name, user_type }
    }
}