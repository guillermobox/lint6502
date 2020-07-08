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

    struct TestCase {
        input: &'static str,
        output: (
            Option<&'static str>,
            Option<&'static str>,
            Option<&'static str>,
        ),
    }

    #[test]
    fn test_creation_from_strings() {
        let testcases = [
            TestCase {
                input: "next: lda ($24),x ; load from the table",
                output: (
                    Some("next"),
                    Some("lda ($24),x"),
                    Some(" load from the table"),
                ),
            },
            TestCase {
                input: "  next  :    lda ($24),x      ; load from the table",
                output: (
                    Some("next"),
                    Some("lda ($24),x"),
                    Some(" load from the table"),
                ),
            },
            TestCase {
                input: ":    instr",
                output: (Some(""), Some("instr"), None),
            },
            TestCase {
                input: "label:",
                output: (Some("label"), None, None),
            },
            TestCase {
                input: "operation",
                output: (None, Some("operation"), None),
            },
            TestCase {
                input: ";comment",
                output: (None, None, Some("comment")),
            },
        ];

        for test in testcases.iter() {
            let line = Line::from(test.input);
            assert_eq!(line.label, test.output.0.map(String::from));
            assert_eq!(line.instruction, test.output.1.map(String::from));
            assert_eq!(line.comment, test.output.2.map(String::from));
        }
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
