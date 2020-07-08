use regex::Regex;

#[derive(Debug, PartialEq)]
pub struct Line {
    label: Option<String>,
    instruction: Option<String>,
    comment: Option<String>,
}

impl From<&str> for Line {
    fn from(line: &str) -> Self {
        let r = Regex::new(
            r"^\s*((?P<label>[^:;]*?)\s*:)?\s*(?P<instruction>[^;]+?)?\s*(;(?P<comment>.+))?$",
        )
        .unwrap();
        let c = r.captures(line).unwrap();
        Self {
            label: c.name("label").map(|x| x.as_str().into()),
            instruction: c.name("instruction").map(|x| x.as_str().into()),
            comment: c.name("comment").map(|x| x.as_str().into()),
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
                label: Some(String::from("next")),
                instruction: Some(String::from("lda ($24),x")),
                comment: Some(String::from(" load from the table"))
            },
            Line::from(line)
        )
    }

    #[test]
    fn test_white_space_is_removed() {
        let line = "  next  :    lda ($24),x      ; load from the table";
        assert_eq!(
            Line {
                label: Some(String::from("next")),
                instruction: Some(String::from("lda ($24),x")),
                comment: Some(String::from(" load from the table"))
            },
            Line::from(line)
        )
    }

    #[test]
    fn test_empty_label_is_found() {
        let line = ":    instr";
        assert_eq!(
            Line {
                label: Some(String::from("")),
                instruction: Some(String::from("instr")),
                comment: None
            },
            Line::from(line)
        )
    }

    #[test]
    fn test_extract_label() {
        assert_eq!(
            Line {
                label: Some(String::from("label")),
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
                instruction: Some(String::from("operation")),
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
                comment: Some(String::from("comment"))
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
                instruction: Some(String::from("beq :+")),
                comment: None
            },
            Line::from("beq :+")
        );
    }
}
