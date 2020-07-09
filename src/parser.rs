use regex::Regex;

#[derive(Debug, PartialEq)]
pub struct Line {
    label: Option<String>,
    instruction: Option<String>,
    comment: Option<String>,
}

impl Default for Line {
    fn default() -> Self {
        Line {
            label: None,
            instruction: None,
            comment: None,
        }
    }
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
        output: Line,
    }

    impl TestCase {
        fn new(input: &'static str) -> Self {
            TestCase {
                input: input,
                output: Line::default(),
            }
        }
        fn label(mut self, value: &'static str) -> Self {
            self.output.label = Some(String::from(value));
            self
        }
        fn instruction(mut self, value: &'static str) -> Self {
            self.output.instruction = Some(String::from(value));
            self
        }
        fn comment(mut self, value: &'static str) -> Self {
            self.output.comment = Some(String::from(value));
            self
        }
        fn assert(&self) {
            let found = Line::from(self.input);
            assert!(
                found == self.output,
                "Test assertion failed\n     Input: '{}'\n  Expected: {:?}\n     Found: {:?}\n",
                self.input,
                self.output,
                found
            )
        }
    }

    #[test]
    fn test_creation_from_strings() {
        let testcases = [
            TestCase::new("next: lda ($24),x ; load from the table")
                .label("next")
                .instruction("lda ($24),x")
                .comment(" load from the table"),
            TestCase::new(":   instr").label("").instruction("instr"),
            TestCase::new("label:").label("label"),
            TestCase::new("operation").instruction("operation"),
            TestCase::new(";comment").comment("comment"),
        ];

        for test in testcases.iter() {
            test.assert();
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
