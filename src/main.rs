use std::collections::HashMap;

fn main() {
    let mut args = std::env::args().skip(1);
    let key = args.next().expect("Key required");
    // expect: Panics if the value is a [None] with a custom panic message provided by msg.
    let value = args.next().unwrap();
    // unwrap: Panics if the self value equals [None].
    println!("The key is '{}' and the value is '{}'", key, value);
    // let contents = format!("{}\t{}\n", key, value);
    // let write_result = std::fs::write("kv.db", contents).unwrap();
    let mut database = Database::new().expect("Failed to create database");
    database.insert(key.to_uppercase(), value.clone());
    // to_uppercase: Returns a new String, so I'm able to use `key` it again.
    // clone: Returns a copy of the value.
    database.insert(key, value);
    database.flush().unwrap();
}

struct Database {
    map: HashMap<String, String>,
}

impl Database {
    fn new() -> Result<Database, std::io::Error> {
        let mut map: HashMap<String, String> = HashMap::new();
        let contents = std::fs::read_to_string("kv.db")?;
        // the `?` operator can only be used in a function that returns `Result` or `Option`
        for line in contents.lines() {
            let mut chunks = line.splitn(2, '\t');
            let key = chunks.next().expect("No key");
            let value = chunks.next().expect("No value");
            map.insert(key.to_owned(), value.to_string());
            // to_owned: Creates owned data from borrowed data, usually by cloning.
            // to_string: Converts the given value to a String.
        }
        Ok(Database { map })
    }
    fn insert(&mut self, key: String, value: String) {
        // Try on this order and use the one that works:
        // &self: immutable borrow
        // &mut self: mutable borrow
        // self: ownership
        self.map.insert(key, value);
    }
    fn flush(self) -> std::io::Result<()> {
        // This function takes ownership of database so database cannot be used anymore.
        let mut contents = String::new();
        for (key, value) in &self.map {
            contents.push_str(key);
            contents.push('\t');
            contents.push_str(value);
            contents.push('\n');
        }
        std::fs::write("kv.db", contents)
    }
}
