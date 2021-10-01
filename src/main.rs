mod library;
pub use library::{
    multiple_option_input::get_multiple_option_input,
    
};

fn main() {
    
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_sign_up_or_log_in() {
        let result: Result<> = get_multiple_option_input::<SignUpOrLogIn>("Welcome. Do you want to sign up or log in");
        if result.0 {
            println!("Sign up");
        } else {
            println!("Log in");
        }
    }
}