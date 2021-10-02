pub mod library;

use std::convert::From;
use std::str::FromStr;
use std::path::PathBuf;

use library::input::InputManager;
use library::memory::MemoryInterface;
use library::memory::User;
use library::program::input::{
    line_reader,
    SignUpOrLogIn,
    LogInData
};
use library::program::memory::{
    FileMemory
};


fn main() {
    let mut input = line_reader();
    let input = InputManager::from( &mut input );
    let memory = FileMemory::from(PathBuf::from_str("./data.json").expect("Error when parsing memory file"));
    run(input, memory);
}

fn run(mut input: InputManager, memory: impl MemoryInterface) -> Option<()> {
    let user: User = {
        let sign_up_or_log_in = input.get_multiple_option_input::<SignUpOrLogIn>("Welcome. Do you want to sign up or log in?")?;
        match sign_up_or_log_in {
            SignUpOrLogIn::LogIn => {
                input.try_until_valid(|input| {
                    let data = input.get_form_input::<LogInData>()?;
                    match memory.log_in(
                        data.name.to_string(),
                        data.password.to_string()
                    ) {
                        Some(user) => Some(Ok(user)),
                        None => Some(Err("There's no user with the given data"))
                    }
                })?
            },
            SignUpOrLogIn::SignUp => todo!("Ask for data to sign up")
        }
    };
    println!("{:#?}", user);
    todo!("Start main menu after obtaining user");
}