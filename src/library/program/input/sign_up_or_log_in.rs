use std::str::FromStr;
use crate::library::input::multiple_option::FromMultipleOptionInput;

// pub fn sign_up_or_log_in() {

// }

pub struct SignUpOrLogIn {
    pub sign_up: bool
}

impl FromStr for SignUpOrLogIn {
    type Err = ();
    fn from_str(input: &str) -> Result<Self, Self::Err> {
        Ok(
            SignUpOrLogIn {
                sign_up: match input {
                    "s" | "S" => true,
                    "l" | "L" => false,
                    _ => return Err(())
                }
            }
        )
    }
}

impl FromMultipleOptionInput for SignUpOrLogIn {
    const OPTIONS: &'static [ (&'static str, &'static str) ] = & [
        ("S", "Sign up"),
        ("L", "Log in")
    ];
}