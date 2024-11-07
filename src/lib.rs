use regex::Regex;
use thiserror::Error;

/// Enum representing the different errors that can occur while parsing an XML tag.
///
/// `InvalidFormat`: Indicates that the XML tag format is invalid.
#[derive(Error, Debug)]
pub enum ParseError {
    #[error("Invalid XML format: {0}")]
    InvalidFormat(String),
}

/// Represents an XML tag.
///
/// This struct contains the name of the tag, its attributes, and a flag
/// indicating whether it is self-closing or not.
pub struct XMLTag {
/// The name of the XML tag.
    pub name: String,

/// The attributes associated with the tag, where each key is the
/// attribute's name, and the value is the attribute's value.
    pub attributes: Vec<(String, String)>,

/// A flag indicating whether the tag is self-closing.
    pub is_self_closing: bool,
}

/// A parser for XML-like tags.
pub struct XMLParser;

impl XMLParser {
/// Parses an XML tag string into an `XMLTag` object.
///
/// This function accepts a string containing an XML tag, such as `<tag name="value" />`,
/// and attempts to parse it into an `XMLTag` struct. It extracts the tag name, attributes,
/// and checks whether the tag is self-closing.
///
/// # Parameters
///
/// - `tag_str`: A string slice that contains the XML tag to be parsed.
///
/// # Returns
///
/// Returns a `Result` with either an `XMLTag` on success or a `ParseError` if the tag is malformed.

    pub fn parse_tag(tag_str: &str) -> Result<XMLTag, ParseError> {
        let re = Regex::new(r"^<(\w+)([^>]*)\s*/?>$").unwrap();

        if let Some(caps) = re.captures(tag_str) {
            let name = caps.get(1).unwrap().as_str().to_string();
            let attributes_str = caps.get(2).unwrap().as_str();
            let is_self_closing = tag_str.ends_with("/>");

            let mut attributes = Vec::new();
            let attr_re = Regex::new(r#"(\w+)\s*=\s*"([^"]*)""#).unwrap();
            for attr_cap in attr_re.captures_iter(attributes_str) {
                let key = attr_cap.get(1).unwrap().as_str().to_string();
                let value = attr_cap.get(2).unwrap().as_str().to_string();
                attributes.push((key, value));
            }

            Ok(XMLTag {
                name,
                attributes,
                is_self_closing,
            })
        } else {
            Err(ParseError::InvalidFormat(tag_str.to_string()))
        }
    }
}
