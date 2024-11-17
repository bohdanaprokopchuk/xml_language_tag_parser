# XML Language Tag Parser

This project implements a parser for XML documents containing language tags.

## Grammar Description

The grammar for parsing XML documents with language tags describes the structure and elements of the XML document, including tags, attributes, and text content. Below are the main grammar rules:

### Grammar Rules

#### `WHITESPACE`

- **Description**: This rule matches whitespace characters, such as spaces, tabs, newlines, and carriage returns.
- **Rule**: `WHITESPACE = _{ " " | "\t" | "\n" | "\r" }`

#### `start`

- **Description**: The starting rule for parsing, which matches the start of input (SOI), followed by an element, and then the end of input (EOI).
- **Rule**: `start = { SOI ~ element ~ EOI }`

#### `element`

- **Description**: An element can either be an opening and closing tag pair or a self-closing tag.
- **Rule**: `element = { open_tag ~ content* ~ close_tag | self_closing_tag }`

#### `open_tag`

- **Description**: An opening tag consists of `<`, followed by the tag name, optional attributes, and ends with `>`.
- **Rule**: `open_tag = { "<" ~ tag_name ~ (WHITESPACE* ~ attributes)? ~ WHITESPACE* ~ ">" }`

#### `close_tag`

- **Description**: A closing tag consists of `</`, followed by the tag name, optional whitespace, and ends with `>`.
- **Rule**: `close_tag = { "</" ~ tag_name ~ WHITESPACE* ~ ">" }`

#### `self_closing_tag`

- **Description**: A self-closing tag is similar to an opening tag but ends with `/ >` instead of `>`.
- **Rule**: `self_closing_tag = { "<" ~ tag_name ~ (WHITESPACE* ~ attributes)? ~ WHITESPACE* ~ "/>" }`

#### `content`

- **Description**: The content inside an element can either be another element or plain text.
- **Rule**: `content = { element | text }`

#### `text`

- **Description**: Text content that matches any character except `<` or `>`.
- **Rule**: `text = { (!("<" | ">") ~ ANY)+ }`

#### `attributes`

- **Description**: A sequence of attributes consisting of one or more individual attributes separated by optional whitespace.
- **Rule**: `attributes = { attribute ~ (WHITESPACE* ~ attribute)* }`

#### `attribute`

- **Description**: An attribute consists of a name, an equals sign (`=`), and a quoted value.
- **Rule**: `attribute = { attr_name ~ "=" ~ "\"" ~ attr_value ~ "\"" }`

#### `attr_name`

- **Description**: The name of an attribute, which can consist of alphanumeric characters, dashes (`-`), underscores (`_`), and colons (`:`).
- **Rule**: `attr_name = @{ (ASCII_ALPHANUMERIC | "-" | "_" | ":")* }`

#### `attr_value`

- **Description**: The value of an attribute, which can be any sequence of characters except for a double quote (`"`).
- **Rule**: `attr_value = @{ (!"\"" ~ ANY)+ }`

#### `tag_name`

- **Description**: The name of a tag, which must start with an alphabetic character, followed by alphanumeric characters, dashes (`-`), or underscores (`_`).
- **Rule**: `tag_name = @{ ASCII_ALPHA ~ (ASCII_ALPHANUMERIC | "-" | "_")* }`
