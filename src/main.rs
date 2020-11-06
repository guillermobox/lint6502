mod directives;
mod instructions;
mod parser;

use instructions::Instruction;

fn main() {
    let arguments = std::env::args().collect::<Vec<String>>();
    if arguments.len() != 2 {
        println!("Please give me just an argument with an instruction")
    } else {
        let op = arguments[1].parse();
        match op {
            Ok(Instruction(mne, adr)) => println!("{:} => {:?} {:?}", arguments[1], mne, adr),
            Err(_) => println!("{} is not an 6502 instruction", arguments[1]),
        }
    }
}
