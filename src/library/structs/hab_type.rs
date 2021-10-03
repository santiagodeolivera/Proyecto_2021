use serde::{ Deserialize, Serialize };

from_input! {

    #[toLowercase]
    bool: multipleOption {
        yes: y => true,
         no: n => false
    };


    HabType: multipleOption {
        habilitation A: HabA => HabType::HabA,
        habilitation B: HabB => HabType::HabB
    }
}


#[derive(Clone, Copy, Deserialize, Serialize, PartialEq, Eq, Hash, Debug)]
pub enum HabType {
    HabA,
    HabB
}