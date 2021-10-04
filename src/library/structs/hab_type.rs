use serde::{ Deserialize, Serialize };

#[derive(Clone, Copy, Deserialize, Serialize, PartialEq, Eq, Hash, Debug)]
pub enum HabType {
    HabA,
    HabB
}