# XML Language Tag Parser 

## Short Description
This project implements an **XML language tag parser** that can recognize and parse XML-like tags with attributes, including self-closing tags. The parser processes XML-like strings, extracts the tag name, attributes, and identifies whether the tag is self-closing. This parser can be integrated into larger systems that need to process XML data in a lightweight and efficient manner.


## Parsing Process
1. **Input**: The parser takes a string that represents an XML-like tag. This tag can have attributes and may or may not be self-closing. For example:
`<tag name="value" attribute="another value"/>`.

2. **Parsing with Regular Expressions**: The parser uses regular expressions to find the parts of the tag:

--Tag Name: The name of the tag, like tag in `<tag>`.
--Attributes: The key-value pairs inside the tag, such as name="value" and attribute="another value".
--Self-Closing Check: It checks if the tag is self-closing, like `<tag />`, by looking for a '/' at the end.
--Extracting Tag Data: After finding the tag, the parser extracts:
The name of the tag.
A list of attributes with their values.
A flag indicating if the tag is self-closing.

3.**Output**: The parser returns the results as a structured TagData object with:
--The tag name.
--The attributes (as key-value pairs).
--A boolean value indicating if it is self-closing.

## Grammar Rules

### Grammar Rules

1. **Tag Structure (`tag`)**:
   - A tag begins with `<`, followed by a tag name, optional attributes, and optionally ends with `/` for self-closing tags.
   - Example: `<tag name="value" />` or `<tag>`
   
2. **Tag Name (`name`)**:
   - The tag name consists of alphanumeric characters.
   - Example: `<tag>`, `<div>`, `<h1>`
   
3. **Attributes (`attribute`)**:
   - Each attribute consists of a `key="value"` pair where the key is alphanumeric, and the value is a string enclosed in double quotes.
   - Example: `<tag attr="value">`
   
4. **Self-Closing Tags**:
   - Tags that end with `/` are self-closing, meaning they do not require a closing tag.
   - Example: `<tag />`

### Regular Expressions

To parse these structures, we use the following regular expressions:

- **Tag**: The regular expression used for matching a tag is:
  ```regex
  ^<(\w+)([^>]*)\s*/?>$

## Example

```rust
use xml_language_tag_parser::XMLParser;

fn main() {
    let tag_str = "<tag name=\"value\" />";
    match XMLParser::parse_tag(tag_str) {
        Ok(tag) => {
            println!("Tag name: {}", tag.name);
            println!("Attributes: {:?}", tag.attributes);
            println!("Is self-closing: {}", tag.is_self_closing);
        }
        Err(e) => println!("Error parsing tag: {}", e),
    }
}

