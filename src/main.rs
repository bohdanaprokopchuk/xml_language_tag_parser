use anyhow::Result;
use std::env;
use xml_language_tag_parser::XMLParser;

fn main() -> Result<()> {
    let args: Vec<String> = env::args().collect();
    if args.len() > 1 {
        match args[1].as_str() {
            "parse" => {
                if args.len() == 3 {
                    let tag_str = &args[2];
                    match XMLParser::parse_tag(tag_str) {
                        Ok(tag) => println!(
                            "Parsed tag: {:?}; Self-closing: {}; Attributes: {:?}",
                            tag.name, tag.is_self_closing, tag.attributes
                        ),
                        Err(e) => eprintln!("Error: {}", e),
                    }
                } else {
                    println!("Usage: xml_language_tag_parser parse <tag>");
                }
            }
            "help" => {
                println!(
                    "xml_language_tag_parser CLI\nCommands:\n - parse <tag>\n - help\n - credits"
                );
            }
            "credits" => {
                println!("xml_language_tag_parser created by Bohdana Prokopchuk.");
            }
            _ => println!("Unknown command. Use 'help' for available commands."),
        }
    } else {
        println!("Use 'help' for available commands.");
    }
    Ok(())
}