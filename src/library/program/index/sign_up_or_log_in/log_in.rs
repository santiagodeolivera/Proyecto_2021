use crate::library::input::InputManager;
use crate::library::memory::MemoryInterface;
use crate::library::structs::User;
use std::convert::Infallible;
use crate::library::input::forms::FromFormInput;
use crate::library::structs::TrimmedStr;

pub fn log_in(input: &mut InputManager, memory: &impl MemoryInterface) -> Option<User> {
    input.try_until_valid(|input| {
        let data = input.get_form_input::<LogInData>()?;
        match memory.log_in(
            data.name,
            data.password
        ) {
            Some(user) => Some(Ok(user)),
            None => Some(Err("There's no user with the given data"))
        }
    })
}

#[derive(Debug)]
struct LogInData {
    pub name: TrimmedStr,
    pub password: TrimmedStr
}

// Parses data from input
// Returns Some(Ok(v)) to return a successful value
// Returns Some(Err(e)) to indicate an invalid input
// Returns None to indicate the user wants to quit
use std::borrow::Cow;
impl FromFormInput for LogInData {
    type Err = Infallible;
    fn from_input(input: &mut InputManager) -> Option<Result<Self, Self::Err>> {
        let name: TrimmedStr = input.get_simple_input("Name: ")?;
        let password: TrimmedStr = input.get_simple_input("Password: ")?;
        Some(
            Ok(
                LogInData { name, password }
            )
        )
    }
    fn error_str(_error: Infallible) -> Cow<'static, str> {
        unimplemented!();
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::library::program::input::line_reader;
    use std::convert::From;

    #[test]
    fn test_log_in() {
        let mut input = line_reader();
        let mut input = InputManager::from(&mut input);
        let data: Option<LogInData> = input.get_form_input();
        println!("{:#?}", data);
    }
}
