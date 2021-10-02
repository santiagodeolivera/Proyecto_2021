use std::convert::Infallible;
use crate::library::input::forms::FromFormInput;
use crate::library::program::input::TrimmedStr;

#[derive(Debug)]
pub struct LogInData {
    pub name: TrimmedStr,
    pub password: TrimmedStr
}

// Parses data from input
// Returns Some(Ok(v)) to return a successful value
// Returns Some(Err(e)) to indicate an invalid input
// Returns None to indicate the user wants to quit
use std::borrow::Cow;
use crate::library::input::InputManager;
impl FromFormInput for LogInData {
    type Err = Infallible;
    fn from_input(input: &mut InputManager) -> Option<Result<Self, Self::Err>> {
        let data = (
            input.get_simple_input::<TrimmedStr>("name: ")?,
            input.get_simple_input::<TrimmedStr>("password: ")?
        );
        Some(
            Ok(
                LogInData {
                    name: data.0,
                    password: data.1
                }
            )
        )
    }
    fn error_str(_error: Infallible) -> Cow<'static, str> {
        unimplemented!();
    }
}