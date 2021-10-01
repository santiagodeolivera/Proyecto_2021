mod library;
pub use library::{
    input::multiple_option::{ get_multiple_option_input, MultipleOptionResult }
};

fn main() {
    
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::io::{ BufRead, BufReader, stdin, Result as IoResult };

    fn line_reader() -> impl Iterator<Item = IoResult<String>> {
        BufReader::new(stdin()).lines()
    }

    #[test]
    fn test_sign_up_or_log_in() {
        use crate::library::program::input::SignUpOrLogIn;

        let mut input = line_reader();
        let result: MultipleOptionResult<SignUpOrLogIn> = get_multiple_option_input("Welcome. Do you want to sign up or log in", &mut input);
        if result {
            println!("Sign up");
        } else {
            println!("Log in");
        }
    }
}