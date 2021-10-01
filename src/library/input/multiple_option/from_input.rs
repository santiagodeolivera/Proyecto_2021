use super::ops_data::OpsData;

pub trait FromMultipleOptionInput: std::str::FromStr {
    const OPTIONS: &'static [ (&'static str, &'static str) ];

    fn options() -> OpsData<'static> {
        OpsData::from(Self::OPTIONS)
    }
}