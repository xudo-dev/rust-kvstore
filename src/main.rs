use std::collections::HashMap;

fn main() {
    let mut arguments = std::env::args().skip(1);
    let command = arguments.next().expect("Command not found");
    if command == "set" {
        let key = arguments.next().expect("Key not found");
        let value = arguments.next().expect("Value not found");
        let mut database = Database::new().expect("Database initialization failed");
        database.set(key, value);
    } else if command == "get" {
        let key = arguments.next().expect("Key not found");
        let mut database = Database::new().expect("Database initialization failed");
        let value = database.get(key);
        println!("Value: {}", value);
    } else {
        panic!("Unknown command");
    }
}

struct Database {
    map: HashMap<String, String>,
}

impl Database {
    fn new() -> Result<Database, std::io::Error> {
        let mut map = HashMap::new();
        let contents = std::fs::read_to_string("kv.db")?;
        for line in contents.lines() {
            let (key, value) = line.split_once('\t').expect("Corrupted database");
            map.insert(key.to_owned(), value.to_owned());
        }
        Ok(Database {
            map
        })
    }

    fn set(&mut self, key: String, value: String) {
        self.map.insert(key, value);
    }

    fn get(&mut self, key: String) -> String {
        self.map.get(&key).expect("Key not found").to_owned()
    }
}

impl Drop for Database {
    fn drop(&mut self) {
        let mut contents = String::new();
        for (key, value) in &self.map {
            contents.push_str(&format!("{}\t{}\n", key, value));
        }
        std::fs::write("kv.db", contents).expect("Unable to write database");
    }
}