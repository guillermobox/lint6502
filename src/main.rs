use regex::Regex;
use std::io::BufRead;
use termion::{color, style};

#[derive(Debug, PartialEq)]
struct Line<'a> {
    label: Option<&'a str>,
    instruction: Option<&'a str>,
    comment: Option<&'a str>,
}

impl Line<'_> {
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

fn extract<'a>(line: &'a str) -> Line<'a> {
    let r = Regex::new(
        r"^\s*((?P<label>[^:;]*?)\s*:)?\s*(?P<instruction>[^;]+?)?\s*(;(?P<comment>.+))?$",
    )
    .unwrap();
    let c = r.captures(line).unwrap();
    Line {
        label: c.name("label").map(|x| x.as_str()),
        instruction: c.name("instruction").map(|x| x.as_str()),
        comment: c.name("comment").map(|x| x.as_str()),
    }
}

fn main() {
    let filepath = std::env::args().nth(1).unwrap(); // yes I know
    let fhandler = std::fs::File::open(&filepath).unwrap();
    let reader = std::io::BufReader::new(fhandler);
    for content in reader.lines() {
        if let Ok(string) = content {
            let compiled = extract(string.as_str());
            compiled.color_display();
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_extract() {
        let line = "next: lda ($24),x ; load from the table";
        assert_eq!(
            Line {
                label: Some("next"),
                instruction: Some("lda ($24),x"),
                comment: Some(" load from the table")
            },
            extract(line)
        )
    }

    #[test]
    fn test_white_space_is_removed() {
        let line = "  next  :    lda ($24),x      ; load from the table";
        assert_eq!(
            Line {
                label: Some("next"),
                instruction: Some("lda ($24),x"),
                comment: Some(" load from the table")
            },
            extract(line)
        )
    }

    #[test]
    fn test_empty_label_is_found() {
        let line = ":    instr";
        assert_eq!(
            Line {
                label: Some(""),
                instruction: Some("instr"),
                comment: None
            },
            extract(line)
        )
    }

    #[test]
    fn test_extract_label() {
        assert_eq!(
            Line {
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
            Line {
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
            Line {
                label: None,
                instruction: None,
                comment: Some("comment")
            },
            extract(";comment")
        );
    }
}
