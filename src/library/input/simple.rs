mod from_input;
pub use from_input::*;

use crate::library::flush_stdout;
use crate::library::input::notify_error;
use crate::library::input::InputManager;
pub fn get_simple_input<T>(msg: &str, input: &mut InputManager) -> Option<T> where T: FromSimpleInput {
    loop {
        print!("\n{}", msg);
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