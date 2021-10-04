use std::fmt::{ Display, Formatter, Result as FmtResult };
use std::hash::{ Hash, Hasher };
use syn::LitStr;

pub struct LitStrWrapper {
    v: LitStr
}

impl Display for LitStrWrapper {
    fn fmt(&self, f: &mut Formatter) -> FmtResult {
        write!(f, "{}", self.v.value())
    }
}

impl Hash for LitStrWrapper {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.v.value().hash(state)
    }
}
