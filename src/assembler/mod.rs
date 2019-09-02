pub mod parsers;

use crate::instruction::Opcode;

/// Token represents different parts of instructions.
#[derive(Debug, PartialEq)]
pub enum Token {
    Opcode(Opcode),
    Register(u8),
    IntegerOperand(i32),
}

/// Representation of a complete assembly instruction.
#[derive(Debug, PartialEq)]
pub struct AssemblyInstruction {
    pub opcode: Token,
    pub operand1: Option<Token>,
    pub operand2: Option<Token>,
    pub operand3: Option<Token>,
}

/// Representation of an Iridium program. Its just a collection of
/// instructions.
#[derive(Debug, PartialEq)]
pub struct Program {
    pub instructions: Vec<AssemblyInstruction>,
}
