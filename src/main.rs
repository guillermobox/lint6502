mod instructions;
mod parser;

use instructions::Instruction;

fn main() {
    match std::env::args().collect::<Vec<String>>()[1]
        .parse()
        .unwrap()
    {
        Instruction::LDA => println!("Load accumulator"),
        Instruction::STA => println!("Store accumulator"),
        _ => println!("Something else"),
    }
}
