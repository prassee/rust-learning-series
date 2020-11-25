mod kvdb;
use std::io::Error;

use kvdb::KVDataBase;

pub(crate) fn main() -> Result<(), Error> {
    let mut args = std::env::args().skip(1);
    let cmd = args.next().expect("cannot find command");
    let mut kv_db = KVDataBase::new("kv.db").expect("Cannot init db");
    match cmd.as_ref() {
        "get" => {
            let key = args.next().expect("key arg not found ");
            Ok(match kv_db.get(&key) {
                Some(value) => println!("value for key {:?} is {:?}", key, value),
                None => println!("value for key not found"),
            })
        }
        "set" => {
            let key = args.next().expect("key arg not found");
            let value = args.next().expect("value arg not found");
            Ok(kv_db.set(&key, &value))
        }
        _ => Err(Error::new(
            std::io::ErrorKind::InvalidInput,
            format!("invalid command {}", cmd),
        )),
    }
}
