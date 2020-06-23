use regex::Regex;
use termion::{color, style};

#[derive(Debug, PartialEq)]
pub struct Line<'a> {
    label: Option<&'a str>,
    instruction: Option<&'a str>,
    comment: Option<&'a str>,
}

impl Line<'_> {
    pub fn color_display(&self) {
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
    pub fn from_str<'a>(line: &'a str) -> Line<'a> {
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
            Line::from_str(line)
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
            Line::from_str(line)
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
            Line::from_str(line)
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
            Line::from_str("label:")
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
            Line::from_str("operation")
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
            Line::from_str(";comment")
        );
    }
}
