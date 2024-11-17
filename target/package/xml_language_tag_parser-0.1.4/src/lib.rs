use pest::Parser;
use pest_derive::Parser;
use thiserror::Error;

/// A parser for XML documents using Pest grammar.
#[derive(Parser)]
#[grammar = "grammar.pest"]
pub struct XMLParser;

/// Represents an XML element, which contains a tag name, attributes, content, and optional text.
#[derive(Debug)]
pub struct Element {
    /// The name of the tag (e.g., "div", "p").
    pub tag_name: String,

    /// The list of attributes associated with the tag.
    pub attributes: Vec<Attribute>,

    /// The content inside the element, which could be other elements or text.
    pub content: Vec<Element>,

    /// The optional text content inside the element, if any.
    pub text: Option<String>,
}

/// Represents an attribute in an XML tag with a name and value.
#[derive(Debug)]
pub struct Attribute {
    /// The name of the attribute (e.g., "id", "class").
    pub name: String,

    /// The value of the attribute (e.g., "main", "footer").
    pub value: String,
}

/// Enum for handling different types of parsing errors.
#[derive(Error, Debug)]
pub enum ParseError {
    /// Error returned if XML parsing fails.
    #[error("Failed to parse XML")]
    PestError(#[from] Box<pest::error::Error<Rule>>),

    /// Error returned when there is an unexpected end of input during parsing.
    #[error("Unexpected end of input")]
    UnexpectedEndOfInput,

    /// Error returned for invalid elements encountered during parsing.
    #[error("Invalid element: {0}")]
    InvalidElement(String),

    /// Error when a closing tag for an element is missing.
    #[error("Missing closing tag for element: {0}")]
    MissingClosingTag(String),

    /// Error when an attribute value is invalid.
    #[error("Invalid attribute value: {0}")]
    InvalidAttributeValue(String),
}

/// Parses the input XML string and returns a vector of `Element`s.
///
/// This function takes a string representing XML content and parses it into a structured
/// representation of XML elements. The function will return an error if the XML is malformed.
///
/// # Arguments
///
/// * `input` - A string slice containing the XML content.
///
/// # Returns
///
/// * `Ok(Vec<Element>)` - A vector of parsed elements if the XML is valid.
/// * `Err(ParseError)` - An error if the XML is invalid or cannot be parsed.
///
/// # Example
///
/// ```
/// let input = "<note><to>Tove</to><from>Jani</from></note>";
/// let elements = parse_xml(input);
/// assert!(elements.is_ok());
/// ```
pub fn parse_xml(input: &str) -> Result<Vec<Element>, ParseError> {
    let pairs = XMLParser::parse(Rule::start, input)
        .map_err(|e| ParseError::PestError(Box::new(e)))?;

    let mut elements = Vec::new();

    for pair in pairs {
        for inner_pair in pair.into_inner() {
            if inner_pair.as_rule() == Rule::element {
                elements.push(parse_element(inner_pair)?);
            }
        }
    }

    Ok(elements)
}

/// Parses an individual XML element from a pair of parsed rules.
///
/// This function processes a single XML element, extracting its tag name, attributes, and content.
/// It will recursively parse nested elements and text content.
///
/// # Arguments
///
/// * `pair` - A parsed pair representing an XML element.
///
/// # Returns
///
/// * `Ok(Element)` - A parsed `Element` struct representing the XML element.
/// * `Err(ParseError)` - An error if the element is invalid or malformed.
///
/// # Example
///
/// ```
/// let pair = ...; // Some parsed XML element
/// let element = parse_element(pair);
/// assert!(element.is_ok());
/// ```
fn parse_element(pair: pest::iterators::Pair<Rule>) -> Result<Element, ParseError> {
    let mut tag_name = String::new();
    let mut attributes = Vec::new();
    let mut content = Vec::new();
    let mut text = None;

    for inner_pair in pair.into_inner() {
        match inner_pair.as_rule() {
            Rule::open_tag => {
                for open_tag_child in inner_pair.into_inner() {
                    match open_tag_child.as_rule() {
                        Rule::tag_name => tag_name = open_tag_child.as_str().to_string(),
                        Rule::attributes => attributes = parse_attributes(open_tag_child)?,
                        _ => {}
                    }
                }
            }
            Rule::content => {
                for content_pair in inner_pair.into_inner() {
                    match content_pair.as_rule() {
                        Rule::element => content.push(parse_element(content_pair)?),
                        Rule::text => {
                            text = Some(content_pair.as_str().to_string());
                        }
                        _ => {}
                    }
                }
            }
            Rule::close_tag => {}
            _ => {}
        }
    }

    if tag_name.is_empty() {
        return Err(ParseError::InvalidElement("Empty tag name".to_string()));
    }

    Ok(Element {
        tag_name,
        attributes,
        content,
        text,
    })
}

/// Parses the attributes of an XML element from a parsed pair of rules.
///
/// This function extracts the attributes for a single XML element, returning them as a list of `Attribute` structs.
///
/// # Arguments
///
/// * `pair` - A parsed pair representing the attributes of an XML element.
///
/// # Returns
///
/// * `Ok(Vec<Attribute>)` - A vector of parsed `Attribute` structs.
/// * `Err(ParseError)` - An error if the attributes are invalid or malformed.
///
/// # Example
///
/// ```
/// let pair = ...; // Some parsed attribute pair
/// let attributes = parse_attributes(pair);
/// assert!(attributes.is_ok());
/// ```
fn parse_attributes(pair: pest::iterators::Pair<Rule>) -> Result<Vec<Attribute>, ParseError> {
    let mut attributes = Vec::new();

    for attribute_pair in pair.into_inner() {
        let mut name = String::new();
        let mut value = String::new();

        for attr_part in attribute_pair.into_inner() {
            match attr_part.as_rule() {
                Rule::attr_name => name = attr_part.as_str().to_string(),
                Rule::attr_value => value = attr_part.as_str().to_string(),
                _ => {}
            }
        }

        if !name.is_empty() && !value.is_empty() {
            attributes.push(Attribute { name, value });
        } else {
            return Err(ParseError::InvalidAttributeValue(name));
        }
    }

    Ok(attributes)
}

