use serde::{Deserialize, Serialize};
use std::error::Error;
use std::fs::{self, read_to_string, write};
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
    fn sort_terms(&mut self) -> Self {
        self.terms
            .sort_by(|a, b| a.term.to_lowercase().cmp(&b.term.to_lowercase()));
        self.clone()
    }

    fn to_file(self, path: &str) -> Result<(), Box<dyn Error>> {
        let contents = to_string(&self)?;
        write(path, contents)?;
        Ok(())
    }
}

fn build_static_page(terms: Terms, output_path: &str) {
    // Generate the HTML output
    let mut html = String::new();
    html.push_str("<html>\n");
    html.push_str("<head>\n");
    html.push_str(r#"<meta charset="utf-8">"#);
    html.push_str("<title>Translated Terms</title>\n");
    html.push_str("</head>\n");
    html.push_str("<body>\n");
    html.push_str("<h1>Translated Terms</h1>\n");
    html.push_str("<table>\n");
    html.push_str("<tr><th>English Term</th><th>中文</th></tr>\n");

    // Iterate over each term in the terms table and append to the HTML output
    for term in terms.terms {
        html.push_str("<tr>");
        html.push_str(&format!("<td>{}</td>", term.term));
        html.push_str(&format!("<td>{}</td>", term.translation));
        html.push_str("</tr>\n");
    }

    html.push_str("</table>\n");
    html.push_str("</body>\n");
    html.push_str("</html>");

    // Write the HTML output to a file
    fs::write(output_path, html).expect("Failed to write terms.html");
}

fn main() -> Result<(), Box<dyn Error>> {
    let path = "terms.toml";

    let terms = Terms::from_file(path)?.sort_terms();
    terms.clone().to_file(path)?;

    build_static_page(terms, "index.html");

    Ok(())
}
