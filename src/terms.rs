use serde::{Deserialize, Serialize};
use std::error::Error;
use std::fs::{read_to_string, write};
use toml::{from_str, to_string};

#[derive(Debug, Deserialize, Serialize, Clone, PartialEq)]
pub struct Term {
    pub term: String,
    pub tags: Vec<String>,
    pub translation: String,
}

#[derive(Debug, Deserialize, Serialize, Clone, PartialEq)]
pub struct Terms {
    pub terms: Vec<Term>,
}

#[derive(Debug, Deserialize, Serialize, PartialEq)]
pub struct Tag {
    pub id: String,
    pub description: String,
}

#[derive(Debug, Deserialize, Serialize, PartialEq)]
pub struct Tags {
    pub tags: Vec<Tag>,
}

impl Terms {
    pub fn load_terms(path: &str) -> Result<Self, Box<dyn Error>> {
        let contents = read_to_string(path)?;
        let terms: Terms = from_str(&contents)?;
        Ok(terms)
    }

    /// takes in terms, load tags and check if there's any illegal tags
    pub fn check_all_tags(terms: &[Term]) -> Result<(), Box<dyn Error>> {
        let contents = read_to_string("tags.toml")?;
        let tags: Tags = from_str(&contents)?;
        for term in terms {
            for tag in &term.tags {
                if !Self::check_tag(tag, &tags.tags) {
                    panic!("Illegal tag '{}' in term '{}'", tag, term.term);
                }
            }
        }
        Ok(())
    }

    pub fn check_tag(tag: &str, tags: &[Tag]) -> bool {
        for valid_tag in tags {
            if valid_tag.id == tag {
                return true;
            }
        }
        false
    }

    /// Sort the terms alphabetically by term
    pub fn sort_terms(&mut self) {
        self.terms
            .sort_by(|a, b| a.term.to_lowercase().cmp(&b.term.to_lowercase()));
    }

    pub fn to_file(&self, path: &str) -> Result<(), Box<dyn Error>> {
        let contents = to_string(self)?;
        write(path, contents)?;
        Ok(())
    }
}
