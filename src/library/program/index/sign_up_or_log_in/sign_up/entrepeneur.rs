use crate::library::input::InputManager;
use crate::library::memory::MemoryInterface;
use crate::library::structs::{ User, EntrepeneurData };

pub fn create_entrepeneur(input: &mut InputManager, memory: &impl MemoryInterface) -> Option<Result<User, &'static str>> {
    let data: EntrepeneurData = input.get_form_input()?;
    match memory.create_entrepeneur(data.clone()) {
        true => Some(Ok(memory.log_in(data.name, data.password).unwrap())),
        false => Some(Err("There's already a user with that name"))
    }
}