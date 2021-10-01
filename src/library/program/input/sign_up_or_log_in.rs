use std::str::FromStr;
use std::io::Result as IoResult;
use crate::library::input::FromInput;
use crate::library::input::multiple_option::FromMultipleOptionInput;
use crate::library::input::multiple_option::get_multiple_option_input;

pub fn sign_up_or_log_in(input: &mut impl Iterator<Item = IoResult<String>>) -> Option<SignUpOrLogIn> {
    get_multiple_option_input("Welcome. Do you want to sign up or log in?", input)
}

#[derive(Debug, PartialEq)]
pub enum SignUpOrLogIn {
    SignUp,
    LogIn
}

impl FromStr for SignUpOrLogIn {
    type Err = ();
    fn from_str(input: &str) -> Result<Self, Self::Err> {
        Ok(
            match input {
                "s" | "S" => SignUpOrLogIn::SignUp,
                "l" | "L" => SignUpOrLogIn::LogIn,
                _ => return Err(())
            }
        )
    }
}

impl FromInput for SignUpOrLogIn {
    fn error_str(_error: ()) -> Cow<'static, str> {
        Cow::Borrowed("Invalid input")
    }
}

use std::borrow::Cow;
impl FromMultipleOptionInput for SignUpOrLogIn {
    const OPTIONS: &'static [ (&'static str, &'static str) ] = & [
        ("S", "Sign up"),
        ("L", "Log in")
    ];
}
