use anyhow::{Context, Result};
use std::env;
use xml_language_tag_parser::parse_xml;

fn print_help() {
    println!("XML Parser CLI - Usage:");
    println!("  cargo run <XML>          Parses an XML string and displays its components.");
    println!("  cargo run -- --help      Displays help information.");
    println!("  cargo run -- --credits   Shows project credits.");
}

fn print_credits() {
    println!("XML Parser by Bohdana Prokopchuk");
}

fn main() -> Result<()> {
    let args: Vec<String> = env::args().collect();

    if args.len() < 2 {
        print_help();
        return Ok(());
    }

    match args[1].as_str() {
        "--help" => {
            print_help();
        }
        "--credits" => {
            print_credits();
        }
        xml => {
            let parsed = parse_xml(xml).context("Failed to parse the provided XML")?;

            if parsed.is_empty() {
                println!("Parsed XML Structure: No elements were parsed.");
            } else {
                println!("Parsed XML Structure:");
                for element in &parsed {
                    println!("{:#?}", element);
                }
            }
        }
    }

    Ok(())
}
