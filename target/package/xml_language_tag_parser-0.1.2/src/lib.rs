use pest::Parser;
use pest_derive::Parser;
use thiserror::Error;

#[derive(Parser)]
#[grammar = "grammar.pest"]
pub struct XMLParser;

#[derive(Debug)]
pub struct Element {
    pub tag_name: String,
    pub attributes: Vec<Attribute>,
    pub content: Vec<Element>,
    pub text: Option<String>,
}

#[derive(Debug)]
pub struct Attribute {
    pub name: String,
    pub value: String,
}

#[derive(Error, Debug)]
pub enum ParseError {
    #[error("Failed to parse XML")]
    PestError(#[from] Box<pest::error::Error<Rule>>),

    #[error("Unexpected end of input")]
    UnexpectedEndOfInput,

    #[error("Invalid element: {0}")]
    InvalidElement(String),

    #[error("Missing closing tag for element: {0}")]
    MissingClosingTag(String),

    #[error("Invalid attribute value: {0}")]
    InvalidAttributeValue(String),
}

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
