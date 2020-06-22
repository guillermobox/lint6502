use regex::Regex;
use std::io::BufRead;
use termion::{color, style};

#[derive(Debug)]
struct Line<'a> {
    file: &'a String,
    position: u32,
    content: String,
}

#[derive(Debug, PartialEq)]
struct CompiledLine<'a> {
    label: Option<&'a str>,
    instruction: Option<&'a str>,
    comment: Option<&'a str>,
}

impl CompiledLine<'_> {
    fn color_display(&self) {
        if let Some(label) = self.label {
            print!("{}{}: ", color::Fg(color::Red), label);
        };
        if let Some(instruction) = self.instruction {
            print!("{}{}", color::Fg(color::Green), instruction);
        };
        if let Some(comment) = self.comment {
            print!("{} ;{}", color::Fg(color::Blue), comment);
        };
        println!("{}", style::Reset);
    }
}

fn extract<'a>(line: &'a str) -> CompiledLine<'a> {
    let r =
        Regex::new(r"^\s*((?P<label>[^:;]*):)?\s*(?P<instruction>[^;]+?)?\s*(;(?P<comment>.+))?$")
            .unwrap();
    let c = r.captures(line).unwrap();
    CompiledLine {
        label: c.name("label").map(|x| x.as_str()),
        instruction: c.name("instruction").map(|x| x.as_str()),
        comment: c.name("comment").map(|x| x.as_str()),
    }
}

fn main() {
    let filepath = std::env::args().nth(1).unwrap(); // yes I know
    let fhandler = std::fs::File::open(&filepath).unwrap();
    let reader = std::io::BufReader::new(fhandler);
    let mut i = 0;
    for content in reader.lines() {
        let line = Line {
            file: &filepath,
            position: i,
            content: content.unwrap(),
        };
        let compiled = extract(line.content.as_str());
        i = i + 1;
        compiled.color_display();
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_extract() {
        let line = "next: lda ($24),x ; load from the table";
        assert_eq!(
            CompiledLine {
                label: Some("next"),
                instruction: Some("lda ($24),x"),
                comment: Some(" load from the table")
            },
            extract(line)
        )
    }

    #[test]
    fn test_extract_label() {
        assert_eq!(
            CompiledLine {
                label: Some("label"),
                instruction: None,
                comment: None
            },
            extract("label:")
        );
    }
    #[test]

    fn test_extract_operation() {
        assert_eq!(
            CompiledLine {
                label: None,
                instruction: Some("operation"),
                comment: None
            },
            extract("operation")
        );
    }
    #[test]

    fn test_extract_comment() {
        assert_eq!(
            CompiledLine {
                label: None,
                instruction: None,
                comment: Some("comment")
            },
            extract(";comment")
        );
    }
}
