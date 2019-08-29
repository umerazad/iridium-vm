use crate::instruction::Opcode;

const MAX_REGISTERS: usize = 32;

#[derive(Default)]
pub struct VM {
    registers: [i32; MAX_REGISTERS],
    pc: usize,
    program: Vec<u8>,
}

impl VM {
    /// Create a new VM instance.
    pub fn new() -> Self {
        VM {
            registers: [0; MAX_REGISTERS],
            pc: 0,
            program: vec![],
        }
    }

    /// Execute the VM instance to completion.
    pub fn run(&mut self) {
        loop {
            if self.pc >= self.program.len() {
                break;
            }

            match self.decode_opcode() {
                Opcode::HLT => {
                    println!("HLT encountered. Terminating.");
                    return;
                }
                _ => {
                    println!("Unrecognized opcode. Terminating");
                    return;
                }
            }
        }
    }

    fn decode_opcode(&mut self) -> Opcode {
        let opcode = Opcode::from(self.program[self.pc]);
        self.pc += 1;
        opcode
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_create_vm() {
        let test_vm = VM::new();
        assert_eq!(test_vm.registers, [0; MAX_REGISTERS]);
    }

    #[test]
    fn test_opcode_hlt() {
        let mut vm = VM::new();
        vm.program = vec![0, 0];
        vm.run();
        assert_eq!(vm.pc, 1);
    }

    #[test]
    fn test_opcode_igl() {
        let mut vm = VM::new();
        vm.program = vec![255];
        vm.run();
        assert_eq!(vm.pc, 1);
    }
}
