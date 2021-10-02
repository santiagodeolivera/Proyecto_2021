use crate::library::structs::{ User, CompanyData, TrimmedStr };

pub trait MemoryInterface {
    fn log_in(&self, name: TrimmedStr, password: TrimmedStr) -> Option<User>;
    fn create_company(&self, data: CompanyData) -> bool;
    fn users(&self) -> Vec<User>;
}