mod sign_up;
use sign_up::sign_up;

mod log_in;
use log_in::log_in;

use std::str::FromStr;
use crate::library::input::simple::FromSimpleInput;
use crate::library::input::multiple_option::FromMultipleOptionInput;
use crate::library::structs::User;
use crate::library::input::InputManager;
use crate::library::memory::MemoryInterface;

pub fn get_user(input: &mut InputManager, memory: &impl MemoryInterface) -> Option<User> {
    let sign_up_or_log_in = input.get_multiple_option_input::<SignUpOrLogIn>("Welcome. Do you want to sign up or log in?")?;
    match sign_up_or_log_in {
        SignUpOrLogIn::LogIn => log_in(input, memory),
        SignUpOrLogIn::SignUp => sign_up(input, memory)
    }
}

#[derive(Debug, PartialEq)]
enum SignUpOrLogIn {
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

#[cfg(test)]
mod tests {
    use super::*;
    use std::convert::From;

    #[test]
    fn test_sign_up_or_log_in() {
        let expected = Some(SignUpOrLogIn::SignUp);

        let mut input = [
            "", "", "S"
        ].iter().map(|s| Ok(String::from(*s)));
        let mut input = InputManager::from( &mut input );
        let actual: Option<SignUpOrLogIn> = input.get_multiple_option_input("Welcome. Do you want to sign up or log in?");

        assert_eq!(expected, actual);
    }
}
