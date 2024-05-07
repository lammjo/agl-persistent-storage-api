use rocksdb::{Options, DB};

fn main() {
    let mut db = setup_db("testpath");
    write_db(&mut db, "a", "test");
    read_db(&mut db, "a");
    destroy_db("testpath");
}

fn setup_db(path:&str) -> DB {
    println!("Set up database");
    let db = DB::open_default(path).unwrap();

    return db;
}

fn destroy_db(path:&str) {
    println!("Destroy database");
    let _ = DB::destroy(&Options::default(), path);
}

fn write_db(db:&mut DB, key:&str, value:&str) {
    println!("Write key {}, value {} to database", key, value);
    db.put(key, value).unwrap();
}

fn read_db(db:&mut DB, key:&str) {
    println!("Retrieve value for key {} from database", key);
    match db.get(key) {
        Ok(Some(value)) => println!("retrieved value {}", String::from_utf8(value).unwrap()),
        Ok(None) => println!("value not found"),
        Err(e) => println!("operational problem encountered: {}", e),
    };
}

#[cfg(test)]
// Unit tests go here
mod tests {
use super::*;

    #[test]
    fn db_testwriteread() {
        let mut db = setup_db("testpath");
        write_db(&mut db, "a", "test");
        read_db(&mut db, "a");
        destroy_db("testpath");
    }
}