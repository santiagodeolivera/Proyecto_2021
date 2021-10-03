mod sign_up_or_log_in;
use sign_up_or_log_in::get_user;

mod main_menu;
use main_menu::main_menu;

use crate::library::input::InputManager;
use crate::library::memory::MemoryInterface;
use crate::library::structs::User;

pub fn run(mut input: InputManager, memory: impl MemoryInterface) -> Option<()> {
    let user: User = get_user(&mut input, &memory)?;
    println!("{:#?}", user);
    main_menu(user)
}