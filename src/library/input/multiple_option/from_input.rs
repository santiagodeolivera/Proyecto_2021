use std::convert::Infallible;
use crate::library::input::multiple_option::OpsData;
use crate::library::input::simple::FromSimpleInput;

pub trait FromMultipleOptionInput: FromSimpleInput {
    const OPTIONS: &'static [ (&'static str, &'static str) ];

    fn options() -> OpsData<'static> {
        OpsData::from(Self::OPTIONS)
    }
}

impl FromSimpleInput for bool {
    type Err = ();
    fn from_input(s: &str) -> Result<Self, Self::Err> {
        match s {
            "Y" | "y" => Ok(true),
            "N" | "n" => Ok(false),
            _ => Err(())
        }
    }
}

impl FromMultipleOptionInput for bool {
    const OPTIONS: &'static [ (&'static str, &'static str) ] = &[
        ("Y", "yes"),
        ("N", "no")
    ];
}

impl FromSimpleInput for Infallible {
    type Err = ();
    fn from_input(_: &str) -> Result<Self, Self::Err> {
        Err(())
    }
}

impl FromMultipleOptionInput for Infallible {
    const OPTIONS: &'static [ (&'static str, &'static str) ] = &[];
}