pub mod parsers;

use crate::instruction::Opcode;

// Make sure that all instructions are 4 bytes even. We are
// intentially using 0xFF instead of 0 as '0' could be a valid
// value for a register # i.e. div $1 $2 will end up encoded as
// div $1 $2 $0.
const PADDING: u8 = 0xFF;

/// Token represents different parts of instructions.
#[derive(Debug, PartialEq)]
pub enum Token {
    Opcode(Opcode),
    Register(u8),
    IntegerOperand(i32),
}

impl Token {
    pub fn to_bytes(t: &Token) -> Vec<u8> {
        match t {
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
        }
    }
}

/// Representation of a complete assembly instruction.
#[derive(Debug, PartialEq)]
pub struct AssemblyInstruction {
    pub opcode: Token,
    pub operand1: Option<Token>,
    pub operand2: Option<Token>,
    pub operand3: Option<Token>,
}

impl AssemblyInstruction {
    pub fn to_bytes(&self) -> Vec<u8> {
        let mut result = Vec::new();
        match &self.opcode {
            Token::Opcode(_) => result.extend(Token::to_bytes(&self.opcode)),
            bad_token => {
                eprintln!("Fetal: {:?} found instead of opcode.", bad_token);
                std::process::exit(1);
            }
        };

        for operand in vec![&self.operand1, &self.operand2, &self.operand3] {
            match operand {
                Some(t) => result.extend(Token::to_bytes(t)),
                None => (),
            }
        }

        // Pad the instructions to make them 4-bytes.
        while result.len() < 4 {
            result.push(PADDING);
        }

        result
    }
}

/// Representation of an Iridium program. Its just a collection of
/// instructions.
#[derive(Debug, PartialEq)]
pub struct Program {
    pub instructions: Vec<AssemblyInstruction>,
}

impl Program {
    pub fn to_bytes(&self) -> Vec<u8> {
        let mut result = vec![];
        for inst in &self.instructions {
            result.append(&mut inst.to_bytes());
        }
        result
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_token_to_bytes() {
        assert_eq!(
            Token::to_bytes(&Token::Opcode(Opcode::LOAD)),
            vec![Opcode::LOAD as u8]
        );

        assert_eq!(
            Token::to_bytes(&Token::Opcode(Opcode::HLT)),
            vec![Opcode::HLT as u8]
        );

        assert_eq!(
            Token::to_bytes(&Token::Opcode(Opcode::JMP)),
            vec![Opcode::JMP as u8]
        );
    }

    #[test]
    fn test_assembly_instruction_to_bytes() {
        let load = AssemblyInstruction {
            opcode: Token::Opcode(Opcode::LOAD),
            operand1: Some(Token::Register(10)),
            operand2: Some(Token::IntegerOperand(99)),
            operand3: None,
        };
        assert_eq!(load.to_bytes(), vec![Opcode::LOAD as u8, 10, 0, 99]);

        let eq = AssemblyInstruction {
            opcode: Token::Opcode(Opcode::EQ),
            operand1: Some(Token::Register(10)),
            operand2: Some(Token::Register(20)),
            operand3: None,
        };
        assert_eq!(eq.to_bytes(), vec![Opcode::EQ as u8, 10, 20, PADDING]);
    }

    #[test]
    fn test_program_to_bytes() {
        let program = Program {
            instructions: vec![
                AssemblyInstruction {
                    opcode: Token::Opcode(Opcode::LOAD),
                    operand1: Some(Token::Register(0)),
                    operand2: Some(Token::IntegerOperand(100)),
                    operand3: None,
                },
                AssemblyInstruction {
                    opcode: Token::Opcode(Opcode::LOAD),
                    operand1: Some(Token::Register(1)),
                    operand2: Some(Token::IntegerOperand(200)),
                    operand3: None,
                },
            ],
        };

        let load_opcode = Opcode::LOAD as u8;
        let program_bytes: Vec<u8> = vec![load_opcode, 0, 0, 100, load_opcode, 1, 0, 200];
        assert_eq!(program.to_bytes(), program_bytes);
    }
}
