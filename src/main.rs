mod instructions;
mod parser;

use instructions::Operation;

fn main() {
    let argument = &std::env::args().collect::<Vec<String>>()[1];
    let op: Operation = argument.parse().unwrap();

    println!("{:20} {:4?} {:?}", argument, op.0, op.1);
}
