use crate::opcode::Opcode;

/// Token represents different parts of instructions.
#[derive(Debug, PartialEq)]
pub enum Token {
    Opcode(Opcode),
    Register(u8),
    IntegerOperand(i32),
    StringOperand(String),
    LabelDeclaration(String),
    LabelUsage(String),
    Directive(String),
}

impl Token {
    pub fn to_bytes(&self) -> Vec<u8> {
        match self {
            Token::Opcode(x) => {
                return vec![*x as u8];
            }
            Token::Register(reg) => {
                return vec![*reg];
            }
            Token::IntegerOperand(v) => {
                let bytes = (*v as u16).to_le_bytes();
                return vec![bytes[1], bytes[0]];
            }
            Token::StringOperand(s) => s.as_bytes().to_vec(),
            _ => unimplemented!(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_token_to_bytes() {
        assert_eq!(
            Token::Opcode(Opcode::LOAD).to_bytes(),
            vec![Opcode::LOAD as u8]
        );

        assert_eq!(
            Token::Opcode(Opcode::HLT).to_bytes(),
            vec![Opcode::HLT as u8]
        );

        assert_eq!(
            Token::Opcode(Opcode::JMP).to_bytes(),
            vec![Opcode::JMP as u8]
        );

        assert_eq!(Token::Register(9).to_bytes(), vec![9]);

        assert_eq!(Token::IntegerOperand(0xFFEE).to_bytes(), vec![0xFF, 0xEE]);

        assert_eq!(
            Token::StringOperand(String::from("AZAD")).to_bytes(),
            vec![0x41, 0x5A, 0x41, 0x44]
        );
    }
}
