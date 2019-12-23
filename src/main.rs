use std::collections::HashMap;

pub struct OWO {
    map: HashMap<&'static str, &'static str>,
}

impl OWO {
    pub fn new() -> Self {
        OWO {
            map: [("UwU", "OwO")].iter().cloned().collect(),
        }
    }

    pub fn uwu(&self) -> &'static str {
        return self.map["UwU"].clone();
    }
}

fn main() {
    let owo = OWO::new();
    println!("{}", owo.uwu());
}
