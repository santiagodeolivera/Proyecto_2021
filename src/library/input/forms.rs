mod from_input;
pub use from_input::*;

use crate::library::input::notify_error;
use crate::library::input::InputManager;
pub fn get_form_input<T>(input: &mut InputManager) -> Option<T> where T: FromFormInput {
    loop {
        match T::from_input(input) {
            None => return None,
            Some(Err(e)) => notify_error(&T::error_str(e), input),
            Some(Ok(r)) => if input.get_multiple_option_input_without_error_message::<bool>(&format!("{}\nIs this ok?", r))? {
                return Some(r)
            }
        }
    }
}
