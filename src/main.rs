use std::io::BufRead;

#[derive(Debug)]
struct Line<'a> {
    file: &'a String,
    position: u32,
    content: String,
}

fn main() {
    let filepath = std::env::args().nth(1).unwrap(); // yes I know
    let fhandler = std::fs::File::open(&filepath).unwrap();
    let reader = std::io::BufReader::new(fhandler);

    for content in reader.lines() {
        let line = Line {
            file: &filepath,
            position: 7,
            content: content.unwrap(),
        };
        println!("{}", line.content);
    }
}
