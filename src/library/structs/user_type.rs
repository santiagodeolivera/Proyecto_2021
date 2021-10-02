use serde::{ Deserialize, Serialize };

#[derive(Deserialize, Serialize, Debug)]
#[serde(rename_all = "lowercase")]
pub enum UserType {
    Admin,
    Entrepeneur,
    Company
}

use std::str::FromStr;
impl FromStr for UserType {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err>{
        Ok(
            match s {
                "A" | "a" => UserType::Admin,
                "E" | "e" => UserType::Entrepeneur,
                "C" | "c" => UserType::Company,
                _ => return Err(())
            }
        )
    }
}

use crate::library::input::simple::FromSimpleInput;
impl FromSimpleInput for UserType {}

use crate::library::input::multiple_option::FromMultipleOptionInput;
impl FromMultipleOptionInput for UserType {
    const OPTIONS: &'static [(&'static str, &'static str)] = &[
        ("A", "Admin"),
        ("E", "Entrepeneur"),
        ("C", "Company")
    ];
}