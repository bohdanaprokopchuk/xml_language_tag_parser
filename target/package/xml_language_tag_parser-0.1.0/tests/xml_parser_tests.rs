use xml_language_tag_parser::XMLParser;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_self_closing_tag() {
        let result = XMLParser::parse_tag("<tag attr=\"value\" />");
        assert!(result.is_ok());
        let tag_data = result.unwrap();
        assert_eq!(tag_data.name, "tag");
        assert_eq!(
            tag_data.attributes,
            vec![("attr".to_string(), "value".to_string())]
        );
        assert!(tag_data.is_self_closing);
    }

    #[test]
    fn test_opening_tag() {
        let result = XMLParser::parse_tag("<tag attr=\"value\">");
        assert!(result.is_ok());
        let tag_data = result.unwrap();
        assert_eq!(tag_data.name, "tag");
        assert_eq!(
            tag_data.attributes,
            vec![("attr".to_string(), "value".to_string())]
        );
        assert!(!tag_data.is_self_closing);
    }

    #[test]
    fn test_invalid_tag() {
        let result = XMLParser::parse_tag("<tag attr=\"value\"");
        assert!(result.is_err());
    }
}
