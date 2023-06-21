use std::error::Error;
use std::fs::{self, File};
use std::io::Write;

use crate::terms::Terms;

fn generate_terms_table(terms: &Terms) -> String {
    let mut table = String::new();
    for term in &terms.terms {
        table.push_str("<tr>");
        table.push_str(&format!("<td>{}</td>", term.term));
        table.push_str(&format!("<td>{}</td>", term.translation));
        // cell for tags
        table.push_str("<td>");
        for tag in &term.tags {
            // Each tag is a span with the class "badge" and a unique class based on the tag's name (id)
            table.push_str(&format!(
                "<span class='badge badge-{}'>{}</span> ",
                tag, tag
            ));
        }
        table.push_str("</td>");
        table.push_str("</tr>\n");
    }
    table
}

pub fn build_static_page(terms: &Terms, output_path: &str) -> Result<(), Box<dyn Error>> {
    let mut template = fs::read_to_string("build/template.html")?;
    let terms_table = generate_terms_table(terms);
    template = template.replace("<!-- TERMS_TABLE_PLACEHOLDER -->", &terms_table);
    let mut output_file = File::create(output_path)?;
    write!(output_file, "{}", template)?;
    Ok(())
}
