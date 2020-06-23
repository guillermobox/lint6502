mod parser;

use std::io::BufRead;

fn main() {
    let filepath = std::env::args().nth(1).unwrap(); // yes I know
    let fhandler = std::fs::File::open(&filepath).unwrap();
    let reader = std::io::BufReader::new(fhandler);
    for content in reader.lines() {
        if let Ok(string) = content {
            let compiled = parser::Line::from_str(string.as_str());
            compiled.color_display();
        }
    }
}
