use regex::Regex;
use std::str::FromStr;

#[derive(Debug, PartialEq)]
pub struct Operation(pub Instruction, pub AddressingMode);

impl FromStr for Operation {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let i: Instruction = s[0..3].parse().unwrap();
        if s.len() == 3 {
            Ok(Operation(i, AddressingMode::Implicit))
        } else {
            Ok(Operation(i, s[4..].parse().unwrap()))
        }
    }
}

#[derive(Debug, PartialEq)]
pub enum AddressingMode {
    Implicit,
    Accumulator,
    Inmediate(String),
    Direct(String),
    IndexedByX(String),
    IndexedByY(String),
    IndirectByX(String),
    IndirectByY(String),
    Indirect(String),
}

impl FromStr for AddressingMode {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let r = Regex::new(r"^$").unwrap();
        if r.is_match(s) {
            return Ok(AddressingMode::Implicit);
        }
        let r = Regex::new(r"^A$").unwrap();
        if r.is_match(s) {
            return Ok(AddressingMode::Accumulator);
        }
        let r = Regex::new(r"^#(.+)$").unwrap();
        if r.is_match(s) {
            let a = r.captures(s).unwrap().get(1).unwrap().as_str();
            return Ok(AddressingMode::Inmediate(a.into()));
        }
        let r = Regex::new(r"^\((.+?),[xX]\)$").unwrap();
        if r.is_match(s) {
            let a = r.captures(s).unwrap().get(1).unwrap().as_str();
            return Ok(AddressingMode::IndirectByX(a.into()));
        }
        let r = Regex::new(r"^\((.+?)\)$").unwrap();
        if r.is_match(s) {
            let a = r.captures(s).unwrap().get(1).unwrap().as_str();
            return Ok(AddressingMode::Indirect(a.into()));
        }
        let r = Regex::new(r"^\((.+?)\),[yY]$").unwrap();
        if r.is_match(s) {
            let a = r.captures(s).unwrap().get(1).unwrap().as_str();
            return Ok(AddressingMode::IndirectByY(a.into()));
        }
        let r = Regex::new(r"^(.+),[xX]$").unwrap();
        if r.is_match(s) {
            let a = r.captures(s).unwrap().get(1).unwrap().as_str();
            return Ok(AddressingMode::IndexedByX(a.into()));
        }
        let r = Regex::new(r"^(.+),[yY]$").unwrap();
        if r.is_match(s) {
            let a = r.captures(s).unwrap().get(1).unwrap().as_str();
            return Ok(AddressingMode::IndexedByY(a.into()));
        }
        return Ok(AddressingMode::Direct(s.to_string()));
    }
}

#[derive(Debug, PartialEq)]
pub enum Instruction {
    ADC,
    AND,
    ASL,
    BCC,
    BCS,
    BEQ,
    BIT,
    BMI,
    BNE,
    BPL,
    BRK,
    BVC,
    BVS,
    CLC,
    CLD,
    CLI,
    CLV,
    CMP,
    CPX,
    CPY,
    DEC,
    DEX,
    DEY,
    EOR,
    INC,
    INX,
    INY,
    JMP,
    JSR,
    LDA,
    LDX,
    LDY,
    LSR,
    NOP,
    ORA,
    PHA,
    PHP,
    PLA,
    PLP,
    ROL,
    ROR,
    RTI,
    RTS,
    SBC,
    SEC,
    SED,
    SEI,
    STA,
    STX,
    STY,
    TAX,
    TAY,
    TSX,
    TXA,
    TXS,
    TYA,
}

impl FromStr for Instruction {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let r = Regex::new(
            r"(?i)^(
                adc|and|asl|bcc|bcs|beq|bit|bmi|bne|bpl|brk|bvc|bvs|clc|cld|cli|clv|cmp|cpx|cpy|dec|dex|dey|eor|inc|inx|iny|jmp|jsr|lda|ldx|ldy|lsr|nop|ora|pha|php|pla|plp|rol|ror|rti|rts|sbc|sec|sed|sei|sta|stx|sty|tax|tay|tsx|txa|txs|tya
            )$",
        )
        .unwrap();
        match r.captures(s) {
            Some(a) => match a.get(1).unwrap().as_str().to_lowercase().as_str() {
                "adc" => Ok(Self::ADC),
                "and" => Ok(Self::AND),
                "asl" => Ok(Self::ASL),
                "bcc" => Ok(Self::BCC),
                "bcs" => Ok(Self::BCS),
                "beq" => Ok(Self::BEQ),
                "bit" => Ok(Self::BIT),
                "bmi" => Ok(Self::BMI),
                "bne" => Ok(Self::BNE),
                "bpl" => Ok(Self::BPL),
                "brk" => Ok(Self::BRK),
                "bvc" => Ok(Self::BVC),
                "bvs" => Ok(Self::BVS),
                "clc" => Ok(Self::CLC),
                "cld" => Ok(Self::CLD),
                "cli" => Ok(Self::CLI),
                "clv" => Ok(Self::CLV),
                "cmp" => Ok(Self::CMP),
                "cpx" => Ok(Self::CPX),
                "cpy" => Ok(Self::CPY),
                "dec" => Ok(Self::DEC),
                "dex" => Ok(Self::DEX),
                "dey" => Ok(Self::DEY),
                "eor" => Ok(Self::EOR),
                "inc" => Ok(Self::INC),
                "inx" => Ok(Self::INX),
                "iny" => Ok(Self::INY),
                "jmp" => Ok(Self::JMP),
                "jsr" => Ok(Self::JSR),
                "lda" => Ok(Self::LDA),
                "ldx" => Ok(Self::LDX),
                "ldy" => Ok(Self::LDY),
                "lsr" => Ok(Self::LSR),
                "nop" => Ok(Self::NOP),
                "ora" => Ok(Self::ORA),
                "pha" => Ok(Self::PHA),
                "php" => Ok(Self::PHP),
                "pla" => Ok(Self::PLA),
                "plp" => Ok(Self::PLP),
                "rol" => Ok(Self::ROL),
                "ror" => Ok(Self::ROR),
                "rti" => Ok(Self::RTI),
                "rts" => Ok(Self::RTS),
                "sbc" => Ok(Self::SBC),
                "sec" => Ok(Self::SEC),
                "sed" => Ok(Self::SED),
                "sei" => Ok(Self::SEI),
                "sta" => Ok(Self::STA),
                "stx" => Ok(Self::STX),
                "sty" => Ok(Self::STY),
                "tax" => Ok(Self::TAX),
                "tay" => Ok(Self::TAY),
                "tsx" => Ok(Self::TSX),
                "txa" => Ok(Self::TXA),
                "txs" => Ok(Self::TXS),
                "tya" => Ok(Self::TYA),
                _ => Err(()),
            },
            None => Err(()),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::AddressingMode;
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

    #[test]
    fn test_addressing_parsing() {
        assert_eq!(Ok(AddressingMode::Implicit), "".parse());
        assert_eq!(
            Ok(AddressingMode::Inmediate("$23".to_string())),
            "#$23".parse()
        );
        assert_eq!(Ok(AddressingMode::Direct("$23".to_string())), "$23".parse());
        assert_eq!(
            Ok(AddressingMode::Inmediate("variable".to_string())),
            "#variable".parse()
        );
        assert_eq!(Ok(AddressingMode::Accumulator), "A".parse());
        assert_eq!(
            Ok(AddressingMode::IndexedByX("$25".to_string())),
            "$25,x".parse()
        );
        assert_eq!(
            Ok(AddressingMode::IndexedByX("$25".to_string())),
            "$25,X".parse()
        );
        assert_eq!(
            Ok(AddressingMode::IndexedByY("$25".to_string())),
            "$25,y".parse()
        );
        assert_eq!(
            Ok(AddressingMode::IndexedByY("$25".to_string())),
            "$25,Y".parse()
        );
        assert_eq!(
            Ok(AddressingMode::Indirect("$aa".to_string())),
            "($aa)".parse()
        );
        assert_eq!(
            Ok(AddressingMode::IndirectByX("$bb".to_string())),
            "($bb,x)".parse()
        );
        assert_eq!(
            Ok(AddressingMode::IndirectByY("$cc".to_string())),
            "($cc),y".parse()
        );
    }
}
