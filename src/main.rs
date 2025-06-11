use std::env;
use std::error::Error;

mod html_builder;
mod terms;

use html_builder::build_static_page;
use terms::{Tags, Terms, ToFile};

fn main() -> Result<(), Box<dyn Error>> {
    let args: Vec<String> = env::args().collect();
    let check_only = args.len() > 1 && args[1] == "--check";

    let terms_path = "terms.toml";
    let tags_path = "tags.toml";
    let mut terms = Terms::load_terms(terms_path)?;
    let mut tags = Tags::load_tags(tags_path)?;

    if check_only {
        // check #1: make sure terms are all sorted
        let expected_terms = terms.clone();
        terms.sort_terms();
        assert!(terms == expected_terms, "terms.toml is not sorted");

        // check #2: make sure all tags are all sorted
        let expected_tags = tags.clone();
        tags.sort_tags();
        assert!(tags == expected_tags, "tags.toml is not sorted");

        // check #3: make sure all tags are valid
        terms.check_all_tags(tags)?;
    } else {
        terms.sort_terms();
        terms.to_file(terms_path)?;
        tags.sort_tags();
        tags.to_file(tags_path)?;

        build_static_page(&terms, "build/index.html")?;
    }

    Ok(())
}
