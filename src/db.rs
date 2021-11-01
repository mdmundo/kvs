use std::collections::HashMap;

pub struct Database {
    map: HashMap<String, String>,
    flush: bool,
}

impl Database {
    pub fn new() -> Result<Database, std::io::Error> {
        let mut map: HashMap<String, String> = HashMap::new();
        let contents = std::fs::read_to_string("kv.db").unwrap_or(String::from(""));
        // the `?` operator can only be used in a function that returns `Result` or `Option`
        // Check: https://doc.rust-lang.org/std/result/#the-question-mark-operator-
        for line in contents.lines() {
            let mut chunks = line.splitn(2, '\t');
            let key = chunks.next().expect("No key: File corrupted");
            let value = chunks.next().expect("No value: File corrupted");
            map.insert(key.to_owned(), value.to_string());
            // to_owned: Creates owned data from borrowed data, usually by cloning.
            // to_string: Converts the given value to a String.
        }
        Ok(Database { map, flush: false })
    }
    pub fn insert(&mut self, key: String, value: String) {
        // Try on this order and use the one that works:
        // &self: immutable borrow
        // &mut self: mutable borrow
        // self: ownership
        self.map.insert(key, value);
    }
    pub fn get(&self, key: &str) -> &str {
        // Try on this order and use the one that works:
        // &self: immutable borrow
        // &mut self: mutable borrow
        // self: ownership
        let value = self.map.get(key);
        value.expect("Value not found")
    }
    pub fn flush(mut self) -> std::io::Result<()> {
        // This function takes ownership of database so database cannot be used anymore.
        self.flush = true;
        do_flush(&self)
    }
}

impl Drop for Database {
    fn drop(&mut self) {
        if !self.flush {
            let _ = do_flush(self);
        }
    }
}

fn do_flush(database: &Database) -> std::io::Result<()> {
    let mut contents = String::new();
    for (key, value) in &database.map {
        contents.push_str(key);
        contents.push('\t');
        contents.push_str(value);
        contents.push('\n');
    }
    std::fs::write("kv.db", contents)
}
