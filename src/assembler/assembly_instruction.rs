use std::fmt;

use super::token::Token;
use super::symbols::SymbolTable;
use crate::opcode::Opcode;

// Make sure that all instructions are 4 bytes even. We are
// intentially using 0xFF instead of 0 as '0' could be a valid
// value for a register # i.e. div $1 $2 will end up encoded as
// div $1 $2 $0.
const PADDING: u8 = 0xFF;

pub const INSTRUCTION_SIZE: u32 = 4;

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
  pub fn to_bytes(&self, _st: &SymbolTable) -> Vec<u8> {
    let mut result = Vec::new();
    match &self.opcode {
      Some(op) => result.extend(op.to_bytes()),
      _ => {
        // For now, only the directives (.code, .asciiz, .data etc.) are the only
        // opcode less instructions that we support.
        assert_eq!(
          true,
          self.has_directive(),
          "Invalid instruction: No opcode found."
        );
      }
    };

    for operand in &[&self.operand1, &self.operand2, &self.operand3] {
      match operand {
        Some(t) => result.extend(t.to_bytes()),
        None => (),
      }
    }

    // Pad the instructions to make them 4-bytes.
    while result.len() < INSTRUCTION_SIZE as usize {
      result.push(PADDING);
    }

    result
  }

  pub fn has_label(&self) -> bool {
    self.label.is_some()
  }

  pub fn get_label(&self) -> Option<String> {
    match &self.label {
      Some(Token::LabelDeclaration(label)) => Some(label.clone()),
      _ => None,
    }
  }

  pub fn has_opcode(&self) -> bool {
    self.opcode.is_some()
  }

  pub fn get_opcode(&self) -> Option<Opcode> {
    match self.opcode {
      Some(Token::Opcode(code)) => Some(code),
      _ => None,
    }
  }

  pub fn has_directive(&self) -> bool {
    self.directive.is_some()
  }

  pub fn get_directive(&self) -> Option<String> {
    match &self.directive {
      Some(Token::Directive(d)) => Some(d.clone()),
      _ => None,
    }
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
    let st = SymbolTable::new();
    let load = AssemblyInstruction {
      opcode: Some(Token::Opcode(Opcode::LOAD)),
      operand1: Some(Token::Register(10)),
      operand2: Some(Token::IntegerOperand(99)),
      ..Default::default()
    };
    assert_eq!(load.to_bytes(&st), vec![Opcode::LOAD as u8, 10, 0, 99]);

    let eq = AssemblyInstruction {
      opcode: Some(Token::Opcode(Opcode::EQ)),
      operand1: Some(Token::Register(10)),
      operand2: Some(Token::Register(20)),
      ..Default::default()
    };
    assert_eq!(eq.to_bytes(&st), vec![Opcode::EQ as u8, 10, 20, PADDING]);
  }

  #[test]
  fn test_opcode_less_instruction() {
    let st = SymbolTable::new();
    let inst = AssemblyInstruction {
      directive: Some(Token::Directive("asciiz".to_string())),
      ..Default::default()
    };

    // A directive doesn't really translate into any bytecode yet.
    // So its all padding.
    assert_eq!(inst.to_bytes(&st), vec![255, 255, 255, 255]);
  }
}
