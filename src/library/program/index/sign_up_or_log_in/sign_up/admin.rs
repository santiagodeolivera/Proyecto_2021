use crate::library::input::InputManager;
use crate::library::memory::MemoryInterface;
use crate::library::structs::{ User, AdminData };

pub fn create_admin(input: &mut InputManager, memory: &impl MemoryInterface) -> Option<Result<User, &'static str>> {
    let data: AdminData = input.get_form_input()?;
    println!("{:#?}", data);
    match memory.create_admin(data.clone()) {
        true => Some(Ok(memory.log_in(data.name, data.password).unwrap())),
        false => Some(Err("There's already a user with that name"))
    }
}