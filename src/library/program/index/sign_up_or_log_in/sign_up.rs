mod admin;
use admin::create_admin;

mod entrepeneur;
use entrepeneur::create_entrepeneur;

mod company;
use company::create_company;

use crate::library::input::InputManager;
use crate::library::memory::MemoryInterface;
use crate::library::structs::User;
use crate::library::structs::UserType;

pub fn sign_up(input: &mut InputManager, memory: &impl MemoryInterface) -> Option<User> {
    input.try_until_valid::<User, &'static str, _>(|input| {
        let user_type: UserType = input.get_multiple_option_input("Insert user type")?;
        match user_type {
            UserType::Admin => create_admin(input, memory),
            UserType::Entrepeneur => create_entrepeneur(input, memory),
            UserType::Company => create_company(input, memory)
        }
    })
}