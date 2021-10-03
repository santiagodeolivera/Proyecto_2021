use std::collections::HashMap;
use serde::{ Deserialize, Serialize };
use crate::library::structs::{ UserType, TrimmedStr, HabType, Location };

#[derive(Deserialize, Serialize, Clone, Debug)]
pub struct EntrepeneurData {
    pub name: TrimmedStr,
    pub password: TrimmedStr,
    pub location: Location,
    pub title: TrimmedStr,
    pub habilitations: HashMap<HabType, u8>
}

use std::convert::Into;
use crate::library::structs::User;
impl Into<User> for EntrepeneurData {
    fn into(self) -> User {
        User::new(self.name, UserType::Entrepeneur)
    }
}

use std::fmt::{ Display, Formatter, Result as FmtResult };
impl Display for EntrepeneurData {
    fn fmt(&self, f: &mut Formatter) -> FmtResult {
        writeln!(f, "name: {}", self.name)?;
        writeln!(f, "password: {}", self.password)?;
        writeln!(f, "location: {}", self.location)?;
        writeln!(f, "title: {}", self.title)
    }
}

use std::borrow::Cow;
use std::convert::Infallible;
use crate::library::input::forms::FromFormInput;
use crate::library::input::InputManager;
impl FromFormInput for EntrepeneurData {
    type Err = Infallible;

    fn from_input(input: &mut InputManager) -> Option<Result<Self, Self::Err>> {
        let name: TrimmedStr = input.get_simple_input("Name: ")?;
        let password: TrimmedStr = input.get_simple_input("Password: ")?;
        let location: Location = input.get_simple_input("Location: ")?;
        let title: TrimmedStr = input.get_simple_input("Title: ")?;
        Some(
            Ok(
                EntrepeneurData {
                    name, password, location, title,
                    habilitations: HashMap::new()
                }
            )
        )
    }

    fn error_str(_error: Infallible) -> Cow<'static, str> {
        unimplemented!();
    }
}
