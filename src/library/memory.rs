use crate::library::structs::{ User, AdminData, CompanyData, EntrepeneurData, TrimmedStr };

pub trait MemoryInterface {
    fn log_in(&self, name: TrimmedStr, password: TrimmedStr) -> Option<User>;
    fn users(&self) -> Vec<User>;
    fn create_admin(&self, data: AdminData) -> bool;
    fn create_company(&self, data: CompanyData) -> bool;
    fn create_entrepeneur(&self, data: EntrepeneurData) -> bool;
}
