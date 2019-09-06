use super::assembly_instruction::AssemblyInstruction;
use super::token::Token;
use super::SymbolTable;
use crate::opcode::Opcode;

/// Representation of an Iridium program. Its just a collection of
/// instructions.
#[derive(Debug, PartialEq)]
pub struct Program {
  pub instructions: Vec<AssemblyInstruction>,
}

impl Program {
  pub fn to_bytes(&self, st: &SymbolTable) -> Vec<u8> {
    let mut result = vec![];
    for inst in &self.instructions {
      result.append(&mut inst.to_bytes(st));
    }
    result
  }
}

#[cfg(test)]
mod tests {
  use super::*;
  #[test]
  fn test_program_to_bytes() {
    let st = SymbolTable::new();
    let program = Program {
      instructions: vec![
        AssemblyInstruction {
          opcode: Some(Token::Opcode(Opcode::LOAD)),
          operand1: Some(Token::Register(0)),
          operand2: Some(Token::IntegerOperand(100)),
          ..Default::default()
        },
        AssemblyInstruction {
          opcode: Some(Token::Opcode(Opcode::LOAD)),
          operand1: Some(Token::Register(1)),
          operand2: Some(Token::IntegerOperand(200)),
          ..Default::default()
        },
      ],
    };

    let load_opcode = Opcode::LOAD as u8;
    let program_bytes: Vec<u8> = vec![load_opcode, 0, 0, 100, load_opcode, 1, 0, 200];
    assert_eq!(program.to_bytes(&st), program_bytes);
  }
}
