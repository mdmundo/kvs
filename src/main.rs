fn main() {
    println!("Hello, world!");
    let mut args = std::env::args().skip(1);
    let key = args.next().expect("Key required");
    // expect: Panics if the value is a [None] with a custom panic message provided by msg.
    let value = args.next().unwrap();
    // unwrap: Panics if the self value equals [None].
    println!("The key is '{}' and the value is '{}'", key, value);
    let contents = format!("{}\t{}\n", key, value);
    let write_result = std::fs::write("kv.db", contents).unwrap();
    let database = Database::new().expect("Failed to create database");
}

struct Database {
    map: std::collections::HashMap<String, String>,
}

impl Database {
    fn new() -> Result<Database, std::io::Error> {
        let mut map: std::collections::HashMap<String, String> = std::collections::HashMap::new();
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
        Ok(Database {
            map: std::collections::HashMap::new(),
        })
    }
}
