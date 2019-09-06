/// Opcode enum represents the opcodes for all the instructions supported by the VM.
/// Each opcode is represented by a u8 in the instruction format.
#[derive(FromPrimitive, Copy, Clone, Debug, PartialEq)]
pub enum Opcode {
    // Halt instruction.
    HLT = 0,

    // Load a value into register.
    LOAD = 1,

    // Add operation. It operates on registers.
    //      ADD $0 $1 $2 where $2 = $0 + $1
    ADD = 2,

    // Multiply. It operates on registers.
    //      MUL $0 $1 $2 where $2 = $0 * $1
    MUL = 3,

    // Subtraction operation. It operates on registers.
    //      SUB $0 $1 $2 where $2 = $0 - $1
    SUB = 4,

    // Division operation. It operates on registers.
    //      DIV $0 $1 $2 where $2 = $0 / $1
    //      and remainder is stored at the VM level in the remainder special register.
    DIV = 5,

    // Absolute Jump. It reads the offset from the operand register.
    JMP = 6,

    // Relative jump forward.
    JMPF = 7,

    // Relative jump backward.
    JMPB = 8,

    // Equal: EQ $0 $1. Result is stored in the VM's equal_flag.
    EQ = 9,

    // Not Equal: NEQ $0 $1. Result is stored in the VM's equal_flag.
    NEQ = 10,

    // Greater Than: GT $0 $1. Result is stored in the VM's equal_flag.
    GT = 11,

    // Greater Than OR Equal To: GTE $0 $1. Result is stored in the VM's equal_flag.
    GTE = 12,

    // Less Than: LT $0 $1. Result is stored in the VM's equal_flag.
    LT = 13,

    // Less Than OR Equal To: LTE $0 $1. Result is stored in the VM's equal_flag.
    LTE = 14,

    // Jump If Equal: JEQ $0. It performs an absolute jump to the value of the register
    // if equal_flag is true.
    JEQ = 15,

    // Jump If Not Equal: JENQ $0. It performs an absolute jump to the value of the register
    // if equal_flag is false.
    JNEQ = 16,

    // Extend heap size: ALLOC $0
    ALOC = 17,

    // Illegal instruction.
    IGL = 255,
}

/// Instruction struct represents an instruction for the VM. We support the following
/// instruction formats. All these instructions are 4 bytes.
///
/// 1. opcode: 8bits
/// 2. opcode: 8bits, register: 8bits
/// 3. opcode: 8bits register: 8bits: operand1: 8bits
/// 4. opcode: 8bits register: 8bits: operand1: 8bits: operand2: 8bits
/// 5. opcode: 8bits register: 8bits: operand1: 16bits
///

impl From<Opcode> for u8 {
    fn from(opcode: Opcode) -> Self {
        opcode as u8
    }
}

impl From<u8> for Opcode {
    fn from(v: u8) -> Self {
        match num::FromPrimitive::from_u8(v) {
            Some(x) => x,
            None => Opcode::IGL,
        }
    }
}

impl From<&str> for Opcode {
    fn from(v: &str) -> Self {
        match v.to_uppercase().as_str() {
            "HLT" => Opcode::HLT,
            "LOAD" => Opcode::LOAD,
            "ADD" => Opcode::ADD,
            "MUL" => Opcode::MUL,
            "SUB" => Opcode::SUB,
            "DIV" => Opcode::DIV,
            "JMP" => Opcode::JMP,
            "JMPF" => Opcode::JMPF,
            "JMPB" => Opcode::JMPB,
            "EQ" => Opcode::EQ,
            "NEQ" => Opcode::NEQ,
            "GT" => Opcode::GT,
            "GTE" => Opcode::GTE,
            "LT" => Opcode::LT,
            "LTE" => Opcode::LTE,
            "JEQ" => Opcode::JEQ,
            "JNEQ" => Opcode::JNEQ,
            "ALOC" => Opcode::ALOC,
            _ => Opcode::IGL,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

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
        assert_eq!(Opcode::JEQ, Opcode::from(15));
        assert_eq!(Opcode::JNEQ, Opcode::from(16));
        assert_eq!(Opcode::ALOC, Opcode::from(17));
    }

    #[test]
    fn test_opcode_as_u8() {
        assert_eq!(Opcode::HLT as u8, 0);
        assert_eq!(Opcode::LOAD as u8, 1);
        assert_eq!(Opcode::ADD as u8, 2);
        assert_eq!(Opcode::MUL as u8, 3);
        assert_eq!(Opcode::SUB as u8, 4);
        assert_eq!(Opcode::DIV as u8, 5);
        assert_eq!(Opcode::JMP as u8, 6);
        assert_eq!(Opcode::JMPF as u8, 7);
        assert_eq!(Opcode::JMPB as u8, 8);
        assert_eq!(Opcode::EQ as u8, 9);
        assert_eq!(Opcode::NEQ as u8, 10);
        assert_eq!(Opcode::GT as u8, 11);
        assert_eq!(Opcode::GTE as u8, 12);
        assert_eq!(Opcode::LT as u8, 13);
        assert_eq!(Opcode::LTE as u8, 14);
        assert_eq!(Opcode::JEQ as u8, 15);
        assert_eq!(Opcode::JNEQ as u8, 16);
        assert_eq!(Opcode::ALOC as u8, 17);
        assert_eq!(Opcode::IGL as u8, 255);
    }

    #[test]
    fn test_opcode_from_str() {
        assert_eq!(Opcode::HLT, Opcode::from("hlt"));
        assert_eq!(Opcode::IGL, Opcode::from("hehehe"));
        assert_eq!(Opcode::LOAD, Opcode::from("load"));
        assert_eq!(Opcode::ADD, Opcode::from("add"));
        assert_eq!(Opcode::MUL, Opcode::from("mul"));
        assert_eq!(Opcode::SUB, Opcode::from("sub"));
        assert_eq!(Opcode::DIV, Opcode::from("div"));
        assert_eq!(Opcode::JMP, Opcode::from("jmp"));
        assert_eq!(Opcode::JMPF, Opcode::from("jmpf"));
        assert_eq!(Opcode::JMPB, Opcode::from("jmpb"));
        assert_eq!(Opcode::EQ, Opcode::from("eq"));
        assert_eq!(Opcode::NEQ, Opcode::from("neq"));
        assert_eq!(Opcode::GT, Opcode::from("gt"));
        assert_eq!(Opcode::GTE, Opcode::from("gte"));
        assert_eq!(Opcode::LT, Opcode::from("lt"));
        assert_eq!(Opcode::LTE, Opcode::from("lte"));
        assert_eq!(Opcode::JEQ, Opcode::from("jeq"));
        assert_eq!(Opcode::JNEQ, Opcode::from("jneq"));
        assert_eq!(Opcode::ALOC, Opcode::from("aloc"));
    }
}
