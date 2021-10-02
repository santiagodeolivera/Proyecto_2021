use std::str::FromStr;
use crate::library::input::simple::FromSimpleInput;
use crate::library::input::multiple_option::FromMultipleOptionInput;

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

impl FromSimpleInput for SignUpOrLogIn {
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
