use serde::{Deserialize, Serialize};
use std::error::Error;
use std::fs::{read_to_string, write};
use toml::{from_str, to_string};

pub trait ToFile: Serialize {
    fn to_file(&self, path: &str) -> Result<(), Box<dyn Error>> {
        let contents = to_string(self)?;
        write(path, contents)?;
        Ok(())
    }
}

impl<T: Serialize> ToFile for T {}

#[derive(Debug, Deserialize, Serialize, Clone, PartialEq)]
pub struct Term {
    pub term: String,
    pub tags: Vec<String>,
    pub translation: String,
    pub acronym: Option<String>,
    pub context: Option<String>,
}

#[derive(Debug, Deserialize, Serialize, Clone, PartialEq)]
pub struct Terms {
    pub terms: Vec<Term>,
}

#[derive(Debug, Deserialize, Serialize, Clone, PartialEq)]
pub struct Tag {
    pub id: String,
    pub description: String,
}

#[derive(Debug, Deserialize, Serialize, Clone, PartialEq)]
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
    pub fn check_all_tags(&self, tags: Tags) -> Result<(), Box<dyn Error>> {
        for term in &self.terms {
            for tag in &term.tags {
                if !tags.check_tag(tag) {
                    panic!("Illegal tag '{}' in term '{}'", tag, term.term);
                }
            }
        }
        Ok(())
    }

    /// Sort the terms alphabetically by term
    pub fn sort_terms(&mut self) {
        self.terms
            .sort_by(|a, b| a.term.to_lowercase().cmp(&b.term.to_lowercase()));
    }
}

impl Tags {
    pub fn load_tags(path: &str) -> Result<Self, Box<dyn Error>> {
        let contents = read_to_string(path)?;
        let tags: Tags = from_str(&contents)?;
        Ok(tags)
    }

    pub fn check_tag(&self, tag: &str) -> bool {
        for valid_tag in &self.tags {
            if valid_tag.id == tag {
                return true;
            }
        }
        false
    }

    pub fn sort_tags(&mut self) {
        self.tags
            .sort_by(|a, b| a.id.to_lowercase().cmp(&b.id.to_lowercase()));
    }
}
