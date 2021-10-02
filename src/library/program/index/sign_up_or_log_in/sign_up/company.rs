use crate::library::input::InputManager;
use crate::library::memory::MemoryInterface;
use crate::library::structs::{ User, CompanyData };

pub fn create_company(input: &mut InputManager, memory: &impl MemoryInterface) -> Option<Result<User, &'static str>> {
    let data: CompanyData = input.get_form_input()?;
    println!("{:#?}", data);
    match memory.create_company(data.clone()) {
        true => Some(Ok(memory.log_in(data.name, data.password).unwrap())),
        false => Some(Err("There's already a user with that name"))
    }
}