use std::env;
use std::fs::{File, OpenOptions};
use std::io::{Read, Write};
use std::path::Path;
use toml;
use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Clone, PartialEq, Eq, Serialize)]
struct Terms {
    term: String,
    translation: String,
}

#[derive(Debug, Deserialize, Serialize)]
struct Root {
    terms: Vec<Terms>,
}

fn main() {
    let args: Vec<String> = env::args().collect();

    let check_only = args.len() > 1 && args[1] == "--check";

    let path = Path::new("../terms.toml");
    let display = path.display();

    let mut file = match File::open(&path) {
        Err(why) => panic!("Couldn't open {}: {}", display, why),
        Ok(file) => file,
    };
    let mut contents = String::new();
    file.read_to_string(&mut contents).expect("Unable to read the file");
    
    let mut root: Root = toml::from_str(&contents).expect("Unable to parse the file");
    
    let mut expected = root.terms.clone();
    expected.sort_by(|a, b| a.term.cmp(&b.term));
    
    if root.terms != expected {
        if check_only {
            panic!("terms.toml is not sorted");
        } else {
            root.terms = expected;

            let mut file = OpenOptions::new()
                .write(true)
                .truncate(true)
                .open(&path)
                .expect("Unable to open file for writing");

            let toml = toml::to_string(&root).expect("Unable to convert to TOML");
            file.write_all(toml.as_bytes()).expect("Unable to write to file");
        }
    }
}
