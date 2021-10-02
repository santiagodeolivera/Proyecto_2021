mod from_input;
pub use from_input::*;

mod ops_data;
pub use ops_data::*;

use crate::library::flush_stdout;
use crate::library::input::notify_error;
use crate::library::input::InputManager;
// Prints a message to the console and then asks for a line of input
pub fn get_multiple_option_input<T>(msg: &str, input: &mut InputManager) -> Option<T> where T: FromMultipleOptionInput {
    loop {
        print!("\n{}\n{}\n>>> ", msg, T::options());
        flush_stdout();
        match input.next() {
            None =>                      panic!("Standard input closed"),
            Some(Err(e)) =>              panic!("Input/Output fatal error:\n{}", e),
            Some(Ok(v)) if &v == "\\" =>   return None,
            Some(Ok(v)) => match T::from_str(&v) {
                Ok(r) => return Some(r),
                Err(e) => notify_error(&T::error_str(e), input)
            }
        };
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::library::program::input::SignUpOrLogIn;
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