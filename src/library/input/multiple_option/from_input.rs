use crate::library::input::multiple_option::OpsData;
use crate::library::input::simple::FromSimpleInput;

pub trait FromMultipleOptionInput: FromSimpleInput {
    const OPTIONS: &'static [ (&'static str, &'static str) ];

    fn options() -> OpsData<'static> {
        OpsData::from(Self::OPTIONS)
    }
}