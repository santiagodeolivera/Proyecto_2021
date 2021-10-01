use crate::library::input::multiple_option::OpsData;
use crate::library::input::FromInput;

pub trait FromMultipleOptionInput: FromInput {
    const OPTIONS: &'static [ (&'static str, &'static str) ];

    fn options() -> OpsData<'static> {
        OpsData::from(Self::OPTIONS)
    }
}