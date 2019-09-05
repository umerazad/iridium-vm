use std::fmt;

use crate::assembler::token::Token;
use crate::instruction::Opcode;

// Make sure that all instructions are 4 bytes even. We are
// intentially using 0xFF instead of 0 as '0' could be a valid
// value for a register # i.e. div $1 $2 will end up encoded as
// div $1 $2 $0.
const PADDING: u8 = 0xFF;

/// Representation of a complete assembly instruction.
#[derive(Debug, PartialEq, Default)]
pub struct AssemblyInstruction {
  pub opcode: Option<Token>,
  pub label: Option<Token>,
  pub directive: Option<Token>,
  pub operand1: Option<Token>,
  pub operand2: Option<Token>,
  pub operand3: Option<Token>,
}

impl AssemblyInstruction {
  pub fn to_bytes(&self) -> Vec<u8> {
    let mut result = Vec::new();
    match &self.opcode {
      Some(op) => result.extend(Token::to_bytes(op)),
      bad_token => {
        eprintln!("Fetal: {:?} found instead of opcode.", bad_token);
        std::process::exit(1);
      }
    };

    for operand in &[&self.operand1, &self.operand2, &self.operand3] {
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

impl fmt::Display for AssemblyInstruction {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    write!(
            f,
            "(Label: {:?} Opcode: {:?} Directive: {:?} Operand #1: {:?} Operand #2: {:?} Operand #3: {:?})",
            self.label, self.opcode, self.directive, self.operand1, self.operand2, self.operand3
        )
  }
}

#[cfg(test)]
mod tests {
  use super::*;
  #[test]
  fn test_assembly_instruction_to_bytes() {
    let load = AssemblyInstruction {
      opcode: Some(Token::Opcode(Opcode::LOAD)),
      operand1: Some(Token::Register(10)),
      operand2: Some(Token::IntegerOperand(99)),
      ..Default::default()
    };
    assert_eq!(load.to_bytes(), vec![Opcode::LOAD as u8, 10, 0, 99]);

    let eq = AssemblyInstruction {
      opcode: Some(Token::Opcode(Opcode::EQ)),
      operand1: Some(Token::Register(10)),
      operand2: Some(Token::Register(20)),
      ..Default::default()
    };
    assert_eq!(eq.to_bytes(), vec![Opcode::EQ as u8, 10, 20, PADDING]);
  }
}
