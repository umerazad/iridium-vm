use crate::instruction::Opcode;

/// Token represents different parts of instructions.
#[derive(Debug, PartialEq)]
pub enum Token {
  Opcode(Opcode),
  Register(u8),
  IntegerOperand(i32),
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
}
