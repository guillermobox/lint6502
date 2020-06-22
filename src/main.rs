use regex::Regex;
use std::io::BufRead;

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
        i = i + 1;
        println!("{:?}", extract(line.content.as_str()));
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
