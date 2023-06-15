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
fn main() -> Result<(), Box<dyn Error>> {
    // Read the TOML file
    let contents = read_to_string("terms.toml")?;

    // Parse the TOML content
    let terms: Terms = from_str(&contents)?;

    // Sort the terms alphabetically by term
    let mut sorted_terms = terms.clone();
    sorted_terms
        .terms
        .sort_by(|a, b| a.term.to_lowercase().cmp(&b.term.to_lowercase()));

    // Generate the sorted TOML content
    let sorted_contents = to_string(&sorted_terms)?;

    // Write the sorted TOML back to the file
    write("terms.toml", sorted_contents)?;

    Ok(())
}
