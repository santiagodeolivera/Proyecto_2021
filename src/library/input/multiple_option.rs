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
        print!("\n{}\n{}>>> ", msg, T::options());
        flush_stdout();
        match input.next() {
            None =>                      panic!("Standard input closed"),
            Some(Err(e)) =>              panic!("Input/Output fatal error:\n{}", e),
            Some(Ok(v)) if &v == "\\" =>   return None,
            Some(Ok(v)) => match T::from_str(v.trim()) {
                Ok(r) => return Some(r),
                Err(e) => notify_error(&T::error_str(e), input)
            }
        };
    }
}
