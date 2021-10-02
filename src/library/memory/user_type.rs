use serde::{ Deserialize, Serialize };

#[derive(Deserialize, Serialize, Debug)]
#[serde(rename_all = "lowercase")]
pub enum UserType {
    Admin,
    Entrepeneur,
    Company
}