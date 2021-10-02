mod user;
pub use user::User;

mod user_type;
pub use user_type::UserType;

pub trait MemoryInterface {
    fn log_in(&self, name: String, password: String) -> Option<User>;
    fn users(&self) -> Vec<User>;
}