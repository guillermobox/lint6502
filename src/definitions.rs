use regex::Regex;
use std::str::FromStr;

#[derive(Debug, PartialEq)]
pub struct Definition {
    label: String,
    expression: String,
}

impl FromStr for Definition {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match Regex::new(r"^(.+?)\s*=\s*(.+?)\s*$").unwrap().captures(s) {
            Some(capture) => Ok(Definition {
                label: capture.get(1).unwrap().as_str().into(),
                expression: capture.get(2).unwrap().as_str().into(),
            }),
            None => Err(()),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::Definition;

    #[test]
    fn test_definition_parsing() {
        assert_eq!(
            Ok(Definition {
                label: "x".into(),
                expression: "1".into()
            }),
            "x = 1".parse()
        );
        assert_eq!(
            Ok(Definition {
                label: "variable".into(),
                expression: "5 + 5".into()
            }),
            "variable   =   5 + 5".parse()
        );
    }
}
