use regex::Regex;
use std::str::FromStr;

#[derive(Debug, PartialEq)]
enum Instruction {
    LDA,
}

impl FromStr for Instruction {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let r = Regex::new(r"(?i)(lda)").unwrap();
        let a = r.captures(s);
        match a {
            Some(_) => Ok(Self::LDA),
            None => Err(()),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::Instruction;

    #[test]
    fn test_instruction_parsing() {
        assert_eq!(Ok(Instruction::LDA), "lda".parse());
        assert_eq!(Ok(Instruction::LDA), "LDA".parse());
        assert_eq!(Ok(Instruction::LDA), "Lda".parse());
        assert_eq!("sta".parse::<Instruction>().is_err(), true);
    }
}
