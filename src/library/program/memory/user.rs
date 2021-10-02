use serde::{ Deserialize, Serialize };
use crate::library::structs::{ User, UserType, TrimmedStr, AdminData, CompanyData, EntrepeneurData };

#[derive(Deserialize, Serialize, Clone)]
pub struct UserList {
    pub admins: Vec<AdminData>,
    pub entrepeneurs: Vec<EntrepeneurData>,
    pub companies: Vec<CompanyData>
}

pub struct UserData {
    pub name: TrimmedStr,
    pub password: TrimmedStr,
    pub user_type: UserType
}

use std::convert::Into;
impl Into<User> for UserData {
    fn into(self) -> User {
        User {
            name: self.name,
            user_type: self.user_type
        }
    }
}

impl UserList {
    pub fn into_basic(self) -> impl Iterator<Item = UserData> {
        let admins_iter = self.admins.into_iter().map(|admin| UserData {
            name: admin.name,
            password: admin.password,
            user_type: UserType::Admin
        });

        let entrepeneurs_iter = self.entrepeneurs.into_iter().map(|ent| UserData {
            name: ent.name,
            password: ent.password,
            user_type: UserType::Entrepeneur
        });

        let companies_iter = self.companies.into_iter().map(|company| UserData {
            name: company.name,
            password: company.password,
            user_type: UserType::Company
        });

        admins_iter.chain(entrepeneurs_iter).chain(companies_iter)
    }

    pub fn into_users(self) -> impl Iterator<Item = User> {
        self.into_basic().map(UserData::into)
    }

    pub fn does_not_exist(&self, name: &TrimmedStr) -> bool {
        !self.clone().into_basic().any(|data| &data.name == name)
    }
}