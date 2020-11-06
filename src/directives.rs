use regex::Regex;
use std::str::FromStr;

#[derive(Debug, PartialEq)]
pub struct Directive {
    operation: String,
    operand: String,
}

impl FromStr for Directive {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match Regex::new(r"^\s*\.([^\s]+)\s+(.+)$").unwrap().captures(s) {
            Some(capture) => Ok(Directive {
                operation: capture.get(1).unwrap().as_str().into(),
                operand: capture.get(2).unwrap().as_str().into(),
            }),
            None => Err(()),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::Directive;

    #[test]
    fn test_directive_parsing() {
        assert_eq!(
            Ok(Directive {
                operation: "byte".into(),
                operand: "$24, $25".into()
            }),
            ".byte $24, $25".parse()
        );
    }
}
