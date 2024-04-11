use regex::Regex;
use std::error::Error;
use std::fs::{self, File};
use std::io::Write;

use crate::terms::Terms;

/// Generates an HTML table containing the terms and their translations.
///
/// # Arguments
///
/// * `terms` - A reference to the `Terms` struct containing the list of terms.
///
/// # Returns
///
/// A string representing the HTML table.
fn generate_terms_table(terms: &Terms) -> String {
    let mut table = String::new();
    for term in &terms.terms {
        table.push_str("<tr>");
        table.push_str(&format!("<td>{}</td>", term.term));
        table.push_str(&format!(
            "<td>{}</td>",
            term.acronym.clone().unwrap_or_default()
        ));
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
        table.push_str(&format!(
            "<td> {} </td>",
            parse_markdown_link(term.context.clone().unwrap_or_default())
        ));
        table.push_str("</td>");
        table.push_str("</tr>\n");
    }
    table
}

/// Parses markdown links in a string and converts them to HTML links.
///
/// # Arguments
///
/// * `context` - The string containing markdown links.
///
/// # Returns
///
/// A string with the markdown links converted to HTML links.
fn parse_markdown_link(context: String) -> String {
    let re = Regex::new(r"\[([^\]]+)\]\(([^)]+)\)").unwrap();
    let mut last_end = 0;
    let mut context_field = String::new();

    for cap in re.captures_iter(&context) {
        let link_text = &cap[1];
        let link = &cap[2];
        let start = cap.get(0).unwrap().start();
        let end = cap.get(0).unwrap().end();

        // Push the text up to the link
        context_field.push_str(&context[last_end..start]);

        // Shorten the link text if it is too long
        let display_text = if link_text.len() > 10 {
            format!("{}...", &link_text[..10])
        } else {
            link_text.to_string()
        };

        // Insert the HTML link
        context_field.push_str(&format!(
            "<a href='{}' target=\"_blank\">{}</a>",
            link, display_text
        ));

        // Update the last_end to the end of the current match
        last_end = end;
    }

    // Append any remaining text after the last link
    context_field.push_str(&context[last_end..]);

    context_field
}

/// Builds a static HTML page using a template and the provided terms.
///
/// # Arguments
///
/// * `terms` - A reference to the `Terms` struct containing the list of terms.
/// * `output_path` - The path to the output file.
///
/// # Returns
///
/// A `Result` indicating success or failure. If successful, `Ok(())` is returned.
pub fn build_static_page(terms: &Terms, output_path: &str) -> Result<(), Box<dyn Error>> {
    let mut template = fs::read_to_string("build/template.html")?;
    let terms_table = generate_terms_table(terms);
    template = template.replace("<!-- TERMS_TABLE_PLACEHOLDER -->", &terms_table);
    let mut output_file = File::create(output_path)?;
    write!(output_file, "{}", template)?;
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_markdown_link() {
        let context = "2 links in text [link](https://example.com) [link2](https://example2.com) and some more text.";
        let expected = "2 links in text <a href='https://example.com' target=\"_blank\">link</a> <a href='https://example2.com' target=\"_blank\">link2</a> and some more text.";
        assert_eq!(parse_markdown_link(context.to_string()), expected);
    }
}
