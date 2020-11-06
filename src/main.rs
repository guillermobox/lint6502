mod directives;
mod instructions;
mod parser;

use directives::Directive;
use instructions::Instruction;
use std::io::BufRead;

fn main() {
    let filepath = std::env::args().nth(1).unwrap(); // yes I know
    let fhandler = std::fs::File::open(&filepath).unwrap();
    let reader = std::io::BufReader::new(fhandler);
    for content in reader.lines() {
        if let Ok(string) = content {
            let compiled = parser::Line::from(string.as_str());
            if let Some(label) = compiled.label {
                println!("Label found {}", label);
            }
            if let Some(command) = compiled.instruction {
                if let Ok(instruction) = command.parse::<Instruction>() {
                    println!("Instruction found {:?}", instruction);
                } else if let Ok(directive) = command.parse::<Directive>() {
                    println!("Directive found {:?}", directive);
                } else {
                    println!("UNKNOWN THING!!! {}", command);
                }
            }
        }
    }
}
