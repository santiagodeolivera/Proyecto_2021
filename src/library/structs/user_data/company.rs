use crate::library::input::InputManager;
use crate::library::memory::MemoryInterface;
use crate::library::structs::{ TrimmedStr, User, UserType, Location };
use serde::{ Deserialize, Serialize };

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct CompanyData {
    pub name: TrimmedStr,
    pub password: TrimmedStr,
    pub location: Location,
    pub title: TrimmedStr
}

use std::convert::Into;
impl Into<User> for CompanyData {
    fn into(self) -> User {
        User::new(self.name, UserType::Company)
    }
}

use std::borrow::Cow;
use std::convert::Infallible;
use crate::library::input::forms::FromFormInput;
impl FromFormInput for CompanyData {
    type Err = Infallible;
    fn from_input(input: &mut InputManager) -> Option<Result<Self, Self::Err>> {
        let name: TrimmedStr = input.get_simple_input("Name: ")?;
        let password: Location = input.get_simple_input("Password: ")?;
        let location: Location = input.get_simple_input("Location: ")?;
        let title: TrimmedStr = input.get_simple_input("Title: ")?;
        Some(
            Ok(
                CompanyData { name, password, location, title }
            )
        )
    }
    fn error_str(_error: Infallible) -> Cow<'static, str> {
        unimplemented!();
    }
}