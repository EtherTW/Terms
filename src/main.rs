use serde::{Deserialize, Serialize};
use std::error::Error;
use std::fs::{read_to_string, write};
use toml::{from_str, to_string};

#[derive(Debug, Deserialize, Serialize, Clone)]
struct Term {
    term: String,
    translation: String,
}
#[derive(Debug, Deserialize, Serialize, Clone)]
struct Terms {
    terms: Vec<Term>,
}

impl Terms {
    fn from_file(path: &str) -> Result<Self, Box<dyn Error>> {
        let contents = read_to_string(path)?;
        let terms: Terms = from_str(&contents)?;
        Ok(terms)
    }

    /// Sort the terms alphabetically by term
    fn sort_terms(&mut self) {
        self.terms
            .sort_by(|a, b| a.term.to_lowercase().cmp(&b.term.to_lowercase()))
    }

    fn to_file(self, path: &str) -> Result<(), Box<dyn Error>> {
        let contents = to_string(&self)?;
        write(path, contents)?;
        Ok(())
    }
}

fn main() -> Result<(), Box<dyn Error>> {
    let path = "terms.toml";

    let mut terms = Terms::from_file(path)?;
    terms.sort_terms();
    terms.to_file(path)?;

    Ok(())
}
