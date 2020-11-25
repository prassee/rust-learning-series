use std::io::prelude::*;
use std::io::Error;

#[derive(Debug)]
pub struct KVDataBase {
    map: std::collections::HashMap<String, String>,
    file: std::fs::File,
}

impl KVDataBase {
    pub fn new(db_file: &str) -> Result<Self, Error> {
        let mut file = std::fs::OpenOptions::new()
            .create(true)
            .write(true)
            .read(true)
            .append(true)
            .open(db_file)
            .expect("cannot open file for writing / appending");
        let mut contents = String::new();
        file.read_to_string(&mut contents)
            .expect("cannot dump to string");
        let mut map = std::collections::HashMap::new();
        for line in contents.lines() {
            let mut x = line.splitn(2, '\t');
            let key = x.next().expect("cannot find key");
            let value = x.next().expect("cannot find value");
            map.insert(key.to_owned(), value.to_owned());
        }
        Ok(Self { map, file })
    }

    pub fn set(&mut self, key: &str, value: &str) {
        self.map.insert(key.to_owned(), value.to_owned());
        self.persist(key, value);
    }

    pub fn get(&mut self, key: &str) -> Option<&String> {
        self.map.get(key)
    }

    fn persist(&mut self, key: &str, value: &str) {
        match writeln!(self.file, "{}\t{}", key, value) {
            Ok(()) => {
                println!("written {:?} {:?}", key, value);
            }
            Err(e) => {
                println!("cannot write {:?}", e.to_string());
            }
        }
    }
}
