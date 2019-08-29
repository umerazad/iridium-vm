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
        let mut is_done = false;
        while !is_done {
            is_done = self.execute_instruction();
        }
    }

    /// Execute one instruction.
    pub fn run_once(&mut self) {
        self.execute_instruction();
    }

    // Executes the next instruction.
    fn execute_instruction(&mut self) -> bool {
        if self.pc >= self.program.len() {
            return true;
        }

        let mut is_done = false;
        match self.decode_opcode() {
            Opcode::HLT => {
                println!("HLT encountered. Terminating.");
                is_done = true;
            }
            Opcode::LOAD => {
                // Load is of the form:
                // LOAD #register, operand

                let reg = self.next_8_bits() as usize;
                let num = self.next_16_bits();
                self.registers[reg] = num as i32;
            }
            Opcode::ADD => {
                let reg1 = self.registers[self.next_8_bits() as usize];
                let reg2 = self.registers[self.next_8_bits() as usize];
                self.registers[self.next_8_bits() as usize] = reg1 + reg2;
            }
            Opcode::MUL => {
                let reg1 = self.registers[self.next_8_bits() as usize];
                let reg2 = self.registers[self.next_8_bits() as usize];
                self.registers[self.next_8_bits() as usize] = reg1 * reg2;
            }

            _ => {
                println!("Unrecognized opcode. Terminating");
                is_done = true;
            }
        }
        is_done
    }

    fn next_8_bits(&mut self) -> u8 {
        let result = self.program[self.pc];
        self.pc += 1;
        result
    }

    fn next_16_bits(&mut self) -> u16 {
        let result =
            ((self.program[self.pc] as i32) << 8 | self.program[self.pc + 1] as i32) as u16;
        self.pc += 2;
        result
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
    fn test_hlt() {
        let mut vm = VM::new();
        vm.program = vec![0, 0];
        vm.run();
        assert_eq!(vm.pc, 1);
    }

    #[test]
    fn test_load() {
        let mut vm = VM::new();
        // LOAD #0 500 in little endian.
        vm.program = vec![1, 0, 1, 244];
        vm.run();
        assert_eq!(vm.registers[0], 500);
    }

    #[test]
    fn test_add() {
        let mut vm = VM::new();
        // LOAD $0 10 -> [1, 0, 0, 10]
        // LOAD $1 20 -> [1, 1, 0, 10]
        // ADD $0 $1 $2 -> [2, 0, 1, 2]
        vm.program = vec![1, 0, 0, 10, 1, 1, 0, 10, 2, 0, 1, 2];
        vm.run();
        assert_eq!(vm.registers[0], 10);
        assert_eq!(vm.registers[1], 10);
        assert_eq!(vm.registers[2], 20);
    }

    #[test]
    fn test_mul() {
        let mut vm = VM::new();
        // LOAD $0 10 -> [1, 0, 0, 10]
        // LOAD $1 20 -> [1, 1, 0, 10]
        // MUL $0 $1 $2 -> [3, 0, 1, 2]
        vm.program = vec![1, 0, 0, 10, 1, 1, 0, 10, 3, 0, 1, 2];
        vm.run();
        assert_eq!(vm.registers[0], 10);
        assert_eq!(vm.registers[1], 10);
        assert_eq!(vm.registers[2], 100);
    }

    #[test]
    fn test_illegal_opcode() {
        let mut vm = VM::new();
        vm.program = vec![255];
        vm.run();
        assert_eq!(vm.pc, 1);
    }
}
