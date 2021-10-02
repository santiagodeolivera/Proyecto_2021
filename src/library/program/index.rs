mod sign_up_or_log_in;
use sign_up_or_log_in::get_user;

use crate::library::input::InputManager;
use crate::library::memory::MemoryInterface;
use crate::library::structs::User;

pub fn run(mut input: InputManager, memory: impl MemoryInterface) -> Option<()> {
    let user: User = get_user(&mut input, &memory)?;
    println!("{:#?}", user);
    todo!("Start main menu after obtaining user");
}