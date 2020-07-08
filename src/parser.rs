use regex::Regex;

#[derive(Debug, PartialEq)]
pub struct Line<'a> {
    label: Option<&'a str>,
    instruction: Option<&'a str>,
    comment: Option<&'a str>,
}

impl<'a> From<&'a str> for Line<'a> {
    fn from(line: &'a str) -> Line<'a> {
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
    use super::Line;

    #[test]
    fn test_extract() {
        let line = "next: lda ($24),x ; load from the table";
        assert_eq!(
            Line {
                label: Some("next"),
                instruction: Some("lda ($24),x"),
                comment: Some(" load from the table")
            },
            Line::from(line)
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
            Line::from(line)
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
            Line::from(line)
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
            Line::from("label:")
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
            Line::from("operation")
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
            Line::from(";comment")
        );
    }
    #[test]
    #[should_panic]
    fn test_relative_jump() {
        assert_eq!(
            Line {
                label: None,
                instruction: Some("beq :+"),
                comment: None
            },
            Line::from("beq :+")
        );
    }
}
