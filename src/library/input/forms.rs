mod from_input;
pub use from_input::*;

use crate::library::input::notify_error;
use crate::library::input::InputManager;
pub fn get_form_input<T>(input: &mut InputManager) -> Option<T> where T: FromFormInput {
    loop {
        match T::from_input(input) {
            None => return None,
            Some(Err(e)) => notify_error(&T::error_str(e), input),
            Some(Ok(r)) => return Some(r)
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::library::program::input::line_reader;
    use crate::library::program::input::LogInData;
    use std::convert::From;

    #[test]
    fn test_log_in() {
        let mut input = line_reader();
        let mut input = InputManager::from(&mut input);
        let data: Option<LogInData> = input.get_form_input();
        println!("{:#?}", data);
    }
}