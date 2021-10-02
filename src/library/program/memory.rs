mod index;
use index::JsonData;

mod user;
use user::UserData;

use std::path::PathBuf;
use std::fs::OpenOptions;
use std::io::{ BufReader, BufWriter };

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
    fn read_json_data(&self) -> JsonData {
        let file = OpenOptions::new().read(true).open(&self.path).expect("Error when opening memory file");
        let reader = BufReader::new(file);
        serde_json::from_reader(reader).expect("Error when parsing JSON")
    }

    fn write_json_data(&self, data: &JsonData) {
        let file = OpenOptions::new().write(true).truncate(true).open(&self.path).expect("Error when opening memory file");
        let writer = BufWriter::new(file);
        serde_json::to_writer(writer, data).expect("Error when writing to memory file");
    }

    fn mut_json_data<T, F>(&self, f: F) -> T where
    F: FnOnce(&mut JsonData) -> T {
        let mut data = self.read_json_data();
        let r = f(&mut data);
        self.write_json_data(&data);
        r
    }
}

use std::convert::Into;
use crate::library::memory::MemoryInterface;
use crate::library::structs::{ User, CompanyData, TrimmedStr };
impl MemoryInterface for FileMemory {
    fn users(&self) -> Vec<User> {
        self.read_json_data().users.into_users().collect()
    }

    fn log_in(&self, name: TrimmedStr, password: TrimmedStr) -> Option<User> {
        self.read_json_data().users.into_basic()
            .filter(|data| data.name == name && data.password == password)
            .map(UserData::into)
            .next()
    }

    fn create_company(&self, data: CompanyData) -> bool {
        self.mut_json_data(|json| {
            let does_not_exist = json.users.does_not_exist(&data.name);
            if does_not_exist {
                json.users.companies.push(data);
            }
            does_not_exist
        })
    }
}
