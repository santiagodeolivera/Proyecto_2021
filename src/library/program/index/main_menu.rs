mod admin;
mod company;
mod entrepeneur;

use crate::library::structs::{ User, UserType };

pub fn main_menu(user: User) -> Option<()> {
    match user.user_type {
        UserType::Admin => admin::main_menu(user.name),
        UserType::Company => company::main_menu(user.name),
        UserType::Entrepeneur => entrepeneur::main_menu(user.name)
    }
}