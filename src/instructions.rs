use regex::Regex;
use std::str::FromStr;

#[derive(Debug, PartialEq)]
enum Instruction {
    LDA,
    STA,
}

impl FromStr for Instruction {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let r = Regex::new(r"(?i)^(lda|sta)$").unwrap();
        match r.captures(s) {
            Some(a) => match a.get(1).unwrap().as_str().to_lowercase().as_str() {
                "lda" => Ok(Self::LDA),
                "sta" => Ok(Self::STA),
                _ => Err(()),
            },
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
        assert_eq!(Ok(Instruction::STA), "sta".parse());
        assert_eq!(Ok(Instruction::STA), "STA".parse());
        assert_eq!(Ok(Instruction::STA), "Sta".parse());
    }
}
