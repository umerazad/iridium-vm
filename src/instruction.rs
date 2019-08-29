/// Opcode enum represents the opcodes for all the instructions supported by the VM.
/// Each opcode is represented by a u8 in the instruction format.
#[derive(Debug, PartialEq)]
pub enum Opcode {
    // Halt instruction.
    HLT,

    // Illegal instruction.
    IGL,

    // Load a value into register.
    LOAD,

    // Add operation. It operates on registers.
    //      ADD $0 $1 $2 where $2 = $0 + $1
    ADD,

    // Multiply. It operates on registers.
    //      MUL $0 $1 $2 where $2 = $0 * $1
    MUL,

    // Subtraction operation. It operates on registers.
    //      SUB $0 $1 $2 where $2 = $0 - $1
    SUB,

    // Division operation. It operates on registers.
    //      DIV $0 $1 $2 where $2 = $0 / $1
    //      and remainder is stored at the VM level in the remainder special register.
    DIV,
}

/// Instruction struct represents an instruction for the VM. We support the following
/// instruction formats.
///
/// 1. opcode: 8bits
/// 2. opcode: 8bits, register: 8bits
/// 3. opcode: 8bits register: 8bits: operand1: 8bits
/// 4. opcode: 8bits register: 8bits: operand1: 8bits: operand2: 8bits
/// 5. opcode: 8bits register: 8bits: operand1: 16bits
///
#[derive(Debug, PartialEq)]
pub struct Instruction {
    opcode: Opcode,
}

impl Instruction {
    pub fn new(opcode: Opcode) -> Instruction {
        Instruction { opcode }
    }
}

impl From<u8> for Opcode {
    fn from(v: u8) -> Self {
        match v {
            0 => Opcode::HLT,
            1 => Opcode::LOAD,
            2 => Opcode::ADD,
            3 => Opcode::MUL,
            4 => Opcode::SUB,
            5 => Opcode::DIV,
            _ => Opcode::IGL,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_create_instruction() {
        let inst = Instruction::new(Opcode::HLT);
        assert_eq!(inst.opcode, Opcode::HLT);
    }

    #[test]
    fn test_opcode_from_u8() {
        assert_eq!(Opcode::HLT, Opcode::from(0));
        assert_eq!(Opcode::LOAD, Opcode::from(1));
        assert_eq!(Opcode::ADD, Opcode::from(2));
        assert_eq!(Opcode::MUL, Opcode::from(3));
        assert_eq!(Opcode::SUB, Opcode::from(4));
        assert_eq!(Opcode::DIV, Opcode::from(5));
    }
}
