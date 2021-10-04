// use std::fmt::{ Display, Formatter, Result as FmtResult };
// use std::hash::{ Hash, Hasher };
// use syn::Ident;

// pub struct IdentWrapper {
//     v: Ident
// }

// impl Display for IdentWrapper {
//     fn fmt(&self, f: &mut Formatter) -> FmtResult {
//         Display::fmt(&self.v, f)
//     }
// }

// impl Hash for IdentWrapper {
//     fn hash<H: Hasher>(&self, state: &mut H) {
//         self.v.hash()
//     }
// }