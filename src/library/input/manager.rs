use std::io::Result as IoResult;
use std::convert::From;

pub struct InputManager<'a> {
    iter: &'a mut dyn Iterator<Item = IoResult<String>>
}

impl Iterator for InputManager<'_> {
    type Item = IoResult<String>;
    fn next(&mut self) -> Option<Self::Item> {
        self.iter.next()
    }
}

impl<'a, I> From<&'a mut I> for InputManager<'a>
    where I: Iterator<Item = IoResult<String>> {
        fn from(v: &'a mut I) -> Self {
            InputManager{
                iter: v as &'a mut dyn Iterator<Item = IoResult<String>>
            }
        }
    }


use crate::library::input::simple::FromSimpleInput;
use crate::library::input::multiple_option::FromMultipleOptionInput;
use crate::library::input::forms::FromFormInput;
use crate::library::input::notify_error;
impl InputManager<'_> {
    pub fn get_simple_input<T: FromSimpleInput>(&mut self, msg: &str) -> Option<T> {
        crate::library::input::simple::get_simple_input(msg, self)
    }

    pub fn get_multiple_option_input<T: FromMultipleOptionInput>(&mut self, msg: &str) -> Option<T> {
        crate::library::input::multiple_option::get_multiple_option_input(msg, self)
    }

    pub fn get_multiple_option_input_without_error_message<T: FromMultipleOptionInput>(&mut self, msg: &str) -> Option<T> {
        crate::library::input::multiple_option::get_multiple_option_input_without_error_message(msg, self)
    }

    pub fn get_form_input<T: FromFormInput>(&mut self) -> Option<T> {
        crate::library::input::forms::get_form_input(self)
    }

    pub fn try_until_valid<T, E, F>(&mut self, mut f: F) -> Option<T> where
    F: FnMut(&mut InputManager) -> Option<Result<T, E>>,
    E: AsRef<str>
{
        loop {
            match f(self) {
                None => return None,
                Some(Err(e)) => notify_error(e.as_ref(), self),
                Some(Ok(r)) => return Some(r)
            }
        }
    }
}