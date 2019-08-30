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

    // Absolute Jump. It reads the offset from the operand register.
    JMP,

    // Relative jump forward.
    JMPF,

    // Relative jump backward.
    JMPB,

    // Equal: EQ $0 $1 .. result is stored in the VM's equal_flag.
    EQ,

    // Not Equal: NEQ $0 $1 .. result is stored in the VM's equal_flag.
    NEQ,

    // Greater Than: GT $0 $1 .. result is stored in the VM's equal_flag.
    GT,

    // Greater Than OR Equal To: GTE $0 $1 .. result is stored in the VM's equal_flag.
    GTE,

    // Less Than: LT $0 $1 .. result is stored in the VM's equal_flag.
    LT,

    // Less Than OR Equal To: LTE $0 $1 .. result is stored in the VM's equal_flag.
    LTE,
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
            6 => Opcode::JMP,
            7 => Opcode::JMPF,
            8 => Opcode::JMPB,
            9 => Opcode::EQ,
            10 => Opcode::NEQ,
            11 => Opcode::GT,
            12 => Opcode::GTE,
            13 => Opcode::LT,
            14 => Opcode::LTE,
            255 | _ => Opcode::IGL,
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
        // Halt
        assert_eq!(Opcode::HLT, Opcode::from(0));

        // Illegal opcode
        assert_eq!(Opcode::IGL, Opcode::from(255));

        // Load/store
        assert_eq!(Opcode::LOAD, Opcode::from(1));

        // Arithmatic ops.
        assert_eq!(Opcode::ADD, Opcode::from(2));
        assert_eq!(Opcode::MUL, Opcode::from(3));
        assert_eq!(Opcode::SUB, Opcode::from(4));
        assert_eq!(Opcode::DIV, Opcode::from(5));

        // Jumps
        assert_eq!(Opcode::JMP, Opcode::from(6));
        assert_eq!(Opcode::JMPF, Opcode::from(7));
        assert_eq!(Opcode::JMPB, Opcode::from(8));

        // Equality related ops.
        assert_eq!(Opcode::EQ, Opcode::from(9));
        assert_eq!(Opcode::NEQ, Opcode::from(10));
        assert_eq!(Opcode::GT, Opcode::from(11));
        assert_eq!(Opcode::GTE, Opcode::from(12));
        assert_eq!(Opcode::LT, Opcode::from(13));
        assert_eq!(Opcode::LTE, Opcode::from(14));
    }
}
