use std::{collections::HashMap, io};

pub struct Commands {
    cmd: HashMap<String, fn()>
}

pub fn test() {
    println!("hello!");
}

impl Commands {
    pub fn new() -> Self {
        return Self {
            cmd: HashMap::new(),
        }
    }

    pub fn insert(&mut self, s: &str, f: fn()) -> io::Result<()> {
        self.cmd.insert(String::from(s), f);
        return Ok(());
    }

    pub fn handle(&self, i: String) -> Option<&fn()> {
        // is this dangerous?
        return self.cmd.get(&i);
    }
}
