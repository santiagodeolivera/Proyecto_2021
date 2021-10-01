mod from_input;
pub use from_input::*;

mod ops_data;
pub use ops_data::*;

use std::io::{ Result as IoResult };
use crate::library::flush_stdout;
use crate::library::input::notify_error;

// Prints a message to the console and then asks for a line of input
pub fn get_multiple_option_input<T, I>(msg: &str, input: &mut I) -> Option<T>
    where T: FromMultipleOptionInput,
          I: Iterator<Item = IoResult<String>> {
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

    #[test]
    fn test_sign_up_or_log_in() {
        let expected = Some(SignUpOrLogIn::SignUp);

        let mut input = [
            "", "", "S"
        ].iter().map(|s| Ok(String::from(*s)));
        let actual: Option<SignUpOrLogIn> = get_multiple_option_input("Welcome. Do you want to sign up or log in?", &mut input);

        assert_eq!(expected, actual);
    }
}