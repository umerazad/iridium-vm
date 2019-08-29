/// Opcode enum represents the opcodes for all the instructions supported by the VM.
#[derive(Debug, PartialEq)]
pub enum Opcode {
    // Halt instruction.
    HLT,

    // Illegal instruction.
    IGL,
}

/// Instruction struct represents an instruction for the VM.
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
        assert_eq!(Opcode::IGL, Opcode::from(1));
    }
}
