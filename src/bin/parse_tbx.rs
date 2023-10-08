use std::fs;

use regex::Regex;
use serde::Deserialize;
use serde_xml_rs::from_str;

#[derive(Debug, Deserialize)]
struct LangSet {
    #[serde(rename = "xmllang")]
    lang: String,
    #[serde(rename = "tig")]
    tig: Tig,
}

#[derive(Debug, Deserialize)]
struct Tig {
    #[serde(rename = "term")]
    term: String,
    #[serde(rename = "termNote")]
    term_note: Option<TermNote>,
}

#[derive(Debug, Deserialize)]
struct TermNote {
    #[serde(rename = "type")]
    note_type: String,
}

fn main() {
    // Specify the file path to your XML file
    let file_path = "Ethereum.org_Translationss_Glossary.tbx";

    // Read the XML data from the file into a string
    let mut xml_str = match fs::read_to_string(file_path) {
        Ok(content) => content,
        Err(err) => {
            eprintln!("Error reading the XML file: {:?}", err);
            return;
        }
    };

    // Define a regular expression pattern to find 'xml:lang'
    let pattern = r#"xml:lang"#;
    let regex = Regex::new(pattern).unwrap();

    // Replace 'xml:lang' with 'xmllang' throughout the XML string
    xml_str = regex.replace_all(&xml_str, "xmllang").to_string();

    let parsed: Result<martif, serde_xml_rs::Error> = from_str(&xml_str);

    match parsed {
        Ok(data) => {
            // Iterate through langSet elements and print zh-TW translations
            for entry in data.text.body.term_entry.iter() {
                let en = entry.lang_set.iter().find(|lang_set| lang_set.lang == "en");
                let zhTW = entry
                    .lang_set
                    .iter()
                    .find(|lang_set| lang_set.lang == "zh-TW");
                if let Some(en) = en {
                    println!(
                        "{} -> {}",
                        en.tig.term,
                        zhTW.map(|lang_set| lang_set.tig.term.to_string())
                            .unwrap_or_default()
                    );
                }
            }
        }
        Err(e) => {
            eprintln!("Error parsing XML: {:?}", e);
        }
    }
}

#[derive(Debug, Deserialize)]
struct martif {
    #[serde(rename = "xmllang")]
    lang: String,
    #[serde(rename = "text")]
    text: Text,
}

#[derive(Debug, Deserialize)]
struct Text {
    #[serde(rename = "body")]
    body: Body,
}

#[derive(Debug, Deserialize)]
struct Body {
    #[serde(rename = "termEntry")]
    term_entry: Vec<TermEntry>,
}

#[derive(Debug, Deserialize)]
struct TermEntry {
    #[serde(rename = "langSet")]
    lang_set: Vec<LangSet>,
}
