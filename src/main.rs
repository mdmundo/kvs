mod db;

fn main() {
    let mut args = std::env::args().skip(1);
    // let action = args.next().expect("Action required");
    // add, rm, get
    let key = args.next().expect("Key required");
    // expect: Panics if the value is a [None] with a custom panic message provided by msg.
    let value = args.next().unwrap();
    // unwrap: Panics if the self value equals [None].
    // println!("The key is '{}' and the value is '{}'", key, value);
    let mut database = db::Database::new().expect("Failed to create database");
    database.insert(key.to_uppercase(), value.clone());
    // to_uppercase: Returns a new String, so I'm able to use `key` it again.
    // clone: Returns a copy of the value.
    database.flush().unwrap();
}
