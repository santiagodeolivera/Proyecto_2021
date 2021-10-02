use serde::{ Deserialize, Serialize };
use crate::library::structs::{ UserType, TrimmedStr };

#[derive(Deserialize, Serialize, Clone, Debug)]
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

use std::fmt::{ Display, Formatter, Result as FmtResult };
impl Display for AdminData {
    fn fmt(&self, f: &mut Formatter) -> FmtResult {
        writeln!(f, "name: {}", self.name)?;
        writeln!(f, "password: {}", self.password)
    }
}

use std::borrow::Cow;
use std::convert::Infallible;
use crate::library::input::forms::FromFormInput;
use crate::library::input::InputManager;
impl FromFormInput for AdminData {
    type Err = Infallible;

    fn from_input(input: &mut InputManager) -> Option<Result<Self, Self::Err>> {
        let name: TrimmedStr = input.get_simple_input("Name: ")?;
        let password: TrimmedStr = input.get_simple_input("Password: ")?;
        Some(
            Ok(
                AdminData { name, password }
            )
        )
    }

    fn error_str(_error: Infallible) -> Cow<'static, str> {
        unimplemented!();
    }
}
