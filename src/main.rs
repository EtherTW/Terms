// main.rs
use std::env;
use std::error::Error;

mod html_builder;
mod terms;

use html_builder::build_static_page;
use terms::{Terms};

fn main() -> Result<(), Box<dyn Error>> {
    let args: Vec<String> = env::args().collect();
    let check_only = args.len() > 1 && args[1] == "--check";

    let path = "terms.toml";
    let mut terms = Terms::from_file(path)?;

    if check_only {
        // check #1: make sure terms are all sorted
        let expected = terms.clone();
        terms.sort_terms();
        if terms != expected {
            panic!("terms.toml is not sorted");
        }
        // more checks to be added
    } else {
        terms.sort_terms();
        terms.to_file(path)?;
        build_static_page(&terms, "build/index.html")?;
    }

    Ok(())
}
