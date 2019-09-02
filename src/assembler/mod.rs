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
    opcode: Token,
    operand1: Option<Token>,
    operand2: Option<Token>,
    operand3: Option<Token>,
}
