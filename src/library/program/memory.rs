mod index;
use index::JsonData;
mod user;
use user::UserData;

use std::path::PathBuf;
use std::fs::OpenOptions;
use std::io::BufReader;

pub struct FileMemory {
    path: PathBuf
}

use std::convert::From;
impl From<PathBuf> for FileMemory {
    fn from(path: PathBuf) -> Self {
        FileMemory { path }
    }
}

impl FileMemory {
    fn json_data(&self) -> JsonData {
        let file = OpenOptions::new().read(true).open(&self.path).expect("Error when opening memory file");
        let reader = BufReader::new(file);
        serde_json::from_reader(reader).expect("Error when parsing JSON")
    }
}

use std::convert::Into;
use crate::library::memory::MemoryInterface;
use crate::library::memory::User;
impl MemoryInterface for FileMemory {
    fn users(&self) -> Vec<User> {
        self.json_data().users.into_iter().map(UserData::into).collect()
    }

    fn log_in(&self, name: String, password: String) -> Option<User> {
        self.json_data().users
            .into_iter()
            .filter(|data| data.name == name && data.password == password)
            .map(UserData::into)
            .next()
    }
}