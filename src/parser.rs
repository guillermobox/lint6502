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

    macro_rules! test_group {
        ($name:ident, $($test:expr),*) => (
            #[test]
            fn $name() {
                for test in [
                    $(
                        $test,
                    )*
                    ].iter() {
                    test.assert();
                }
            }
        )
    }

    struct TestCase {
        input: &'static str,
        output: Line,
        failer: bool,
    }

    impl TestCase {
        fn new(input: &'static str) -> Self {
            TestCase {
                input: input,
                output: Line::default(),
                failer: false,
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
        fn should_fail(mut self) -> Self {
            self.failer = true;
            self
        }
        fn assert(&self) {
            let found = Line::from(self.input);
            match self.failer {
                false => assert!(
                    found == self.output,
                    "Test assertion failed\n     Input: '{}'\n  Expected: {:?}\n     Found: {:?}\n",
                    self.input,
                    self.output,
                    found
                ),
                true => assert!(
                    found != self.output,
                    "Test assertion should fail but it did not\n         Input: '{}'\n  Not expected: {:?}\n         Found: {:?}\n",
                    self.input,
                    self.output,
                    found
                )
            }
        }
    }

    test_group!(
        test_creation_from_strings,
        TestCase::new("next: lda ($24),x ; load from the table")
            .label("next")
            .instruction("lda ($24),x")
            .comment(" load from the table"),
        TestCase::new(":   instr").label("").instruction("instr"),
        TestCase::new("label:").label("label"),
        TestCase::new("operation").instruction("operation"),
        TestCase::new(";comment").comment("comment"),
        TestCase::new("beq :+").instruction("beq :+").should_fail()
    );

    test_group!(
        test_empty_space_removal,
        TestCase::new("  a:").label("a"),
        TestCase::new("  a  :").label("a"),
        TestCase::new("a  :").label("a"),
        TestCase::new("a:").label("a"),
        TestCase::new(":").label(""),
        TestCase::new(":   ").label(""),
        TestCase::new(";   co").comment("   co"),
        TestCase::new(";co").comment("co"),
        TestCase::new(";co  ").comment("co").should_fail()
    );
}
