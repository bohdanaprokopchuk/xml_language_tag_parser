use anyhow::{Context, Result};
use pest::Parser;
use xml_language_tag_parser::{Rule, XMLParser};

#[test]
fn test_whitespace_space() -> Result<()> {
    let result = XMLParser::parse(Rule::start, " <note></note> ");
    result.context("Failed to parse XML with surrounding spaces")?;
    Ok(())
}

#[test]
fn test_whitespace_tab() -> Result<()> {
    let result = XMLParser::parse(Rule::start, "\t<note></note>\t");
    result.context("Failed to parse XML with surrounding tabs")?;
    Ok(())
}

#[test]
fn test_whitespace_newline() -> Result<()> {
    let result = XMLParser::parse(Rule::start, "\n<note></note>\n");
    result.context("Failed to parse XML with surrounding newlines")?;
    Ok(())
}

#[test]
fn test_whitespace_carriage_return() -> Result<()> {
    let result = XMLParser::parse(Rule::start, "\r<note></note>\r");
    result.context("Failed to parse XML with surrounding carriage returns")?;
    Ok(())
}

#[test]
fn test_element_with_attributes() -> Result<()> {
    let result = XMLParser::parse(
        Rule::start,
        r#"<car brand="Tesla" model="Model S">Electric</car>"#,
    );
    result.context("Failed to parse element with attributes")?;
    Ok(())
}

#[test]
fn test_element_self_closing_tag() -> Result<()> {
    let result = XMLParser::parse(Rule::start, r#"<note/>"#);
    result.context("Failed to parse self-closing tag")?;
    Ok(())
}

#[test]
fn test_element_invalid_closing_tag() {
    let result = XMLParser::parse(Rule::start, "<note><to>HI</note>");
    assert!(result.is_err(), "Invalid closing tag should fail");
}

#[test]
fn test_text_valid() -> Result<()> {
    let result = XMLParser::parse(Rule::start, "<note>Hello, how are you?</note>");
    result.context("Failed to parse valid text inside tags")?;
    Ok(())
}

#[test]
fn test_text_invalid_symbols() {
    let result = XMLParser::parse(Rule::start, "<note>Text with < and > symbols</note>");
    assert!(
        result.is_err(),
        "Text containing invalid symbols should fail"
    );
}

#[test]
fn test_attributes_multiple_attributes() -> Result<()> {
    let result = XMLParser::parse(
        Rule::start,
        r#"<person name="John" age="25" city="New York"></person>"#,
    );
    result.context("Failed to parse element with multiple attributes")?;
    Ok(())
}

#[test]
fn test_attributes_missing_value() {
    let result = XMLParser::parse(Rule::start, r#"<note><to name=Alice>Hi</to></note>"#);
    assert!(result.is_err(), "Attribute without value should fail");
}

#[test]
fn test_missing_closing_tag() {
    let result = XMLParser::parse(Rule::start, "<note><to>Alice</to>");
    assert!(result.is_err(), "Missing closing tag should fail");
}

#[test]
fn test_invalid_element_name() {
    let result = XMLParser::parse(Rule::start, "<1note><to>Alice</to></1note>");
    assert!(
        result.is_err(),
        "Element name starting with a digit should fail"
    );
}

#[test]
fn test_missing_opening_tag() {
    let result = XMLParser::parse(Rule::start, "Alice</note>");
    assert!(result.is_err(), "Missing opening tag should fail");
}
