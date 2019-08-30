use crate::instruction::Opcode;

const MAX_REGISTERS: usize = 32;

#[derive(Default, Debug)]
pub struct VM {
    // Logical registers.
    registers: [i32; MAX_REGISTERS],

    // Program counter that tracks which instruction is to be executed next.
    pc: usize,

    // Bytecode of the program.
    program: Vec<u8>,

    // Tracks the remainder of the integer division operation.
    remainder: u32,

    // Tracks the result of the last comparison operation.
    equal_flag: bool,
}

impl VM {
    /// Create a new VM instance.
    pub fn new() -> Self {
        VM {
            registers: [0; MAX_REGISTERS],
            pc: 0,
            program: vec![],
            remainder: 0,
            equal_flag: false,
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
            Opcode::SUB => {
                let reg1 = self.registers[self.next_8_bits() as usize];
                let reg2 = self.registers[self.next_8_bits() as usize];
                self.registers[self.next_8_bits() as usize] = reg1 - reg2;
            }
            Opcode::MUL => {
                let reg1 = self.registers[self.next_8_bits() as usize];
                let reg2 = self.registers[self.next_8_bits() as usize];
                self.registers[self.next_8_bits() as usize] = reg1 * reg2;
            }
            Opcode::DIV => {
                let reg1 = self.registers[self.next_8_bits() as usize];
                let reg2 = self.registers[self.next_8_bits() as usize];
                self.registers[self.next_8_bits() as usize] = reg1 / reg2;
                self.remainder = (reg1 % reg2) as u32;
            }
            Opcode::JMP => {
                let target = self.registers[self.next_8_bits() as usize];
                self.pc = target as usize;
            }
            Opcode::JMPF => {
                let target = self.registers[self.next_8_bits() as usize];
                self.pc += target as usize;
            }
            Opcode::JMPB => {
                let target = self.registers[self.next_8_bits() as usize];
                self.pc -= target as usize;
            }

            // Equality related instructions are kind of special given that they don't
            //
            // consumes all 4 bytes (like ADD/SUB) nor it manipulates the
            // PC (JMP etc) so we'll skip over the next byte to make the instruction
            // length evenly 4.
            //
            Opcode::EQ => {
                let r1 = self.registers[self.next_8_bits() as usize];
                let r2 = self.registers[self.next_8_bits() as usize];

                if r1 == r2 {
                    self.equal_flag = true;
                } else {
                    self.equal_flag = false;
                }

                // Skip over next byte to align the PC with 4 byte.
                self.next_8_bits();
            }
            Opcode::NEQ => {
                let r1 = self.registers[self.next_8_bits() as usize];
                let r2 = self.registers[self.next_8_bits() as usize];

                if r1 != r2 {
                    self.equal_flag = true;
                } else {
                    self.equal_flag = false;
                }

                // Skip over next byte to align the PC with 4 byte.
                self.next_8_bits();
            }
            Opcode::GT => {
                let r1 = self.registers[self.next_8_bits() as usize];
                let r2 = self.registers[self.next_8_bits() as usize];

                if r1 > r2 {
                    self.equal_flag = true;
                } else {
                    self.equal_flag = false;
                }

                // Skip over next byte to align the PC with 4 byte.
                self.next_8_bits();
            }
            Opcode::GTE => {
                let r1 = self.registers[self.next_8_bits() as usize];
                let r2 = self.registers[self.next_8_bits() as usize];

                if r1 >= r2 {
                    self.equal_flag = true;
                } else {
                    self.equal_flag = false;
                }

                // Skip over next byte to align the PC with 4 byte.
                self.next_8_bits();
            }
            Opcode::LT => {
                let r1 = self.registers[self.next_8_bits() as usize];
                let r2 = self.registers[self.next_8_bits() as usize];

                if r1 < r2 {
                    self.equal_flag = true;
                } else {
                    self.equal_flag = false;
                }

                // Skip over next byte to align the PC with 4 byte.
                self.next_8_bits();
            }
            Opcode::LTE => {
                let r1 = self.registers[self.next_8_bits() as usize];
                let r2 = self.registers[self.next_8_bits() as usize];

                if r1 <= r2 {
                    self.equal_flag = true;
                } else {
                    self.equal_flag = false;
                }

                // Skip over next byte to align the PC with 4 byte.
                self.next_8_bits();
            }
            Opcode::JEQ => {
                let target = self.registers[self.next_8_bits() as usize];
                if self.equal_flag {
                    self.pc = target as usize;
                }
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
        // LOAD $1 10 -> [1, 1, 0, 10]
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
        // LOAD $1 10 -> [1, 1, 0, 10]
        // MUL $0 $1 $2 -> [3, 0, 1, 2]
        vm.program = vec![1, 0, 0, 10, 1, 1, 0, 10, 3, 0, 1, 2];
        vm.run();
        assert_eq!(vm.registers[0], 10);
        assert_eq!(vm.registers[1], 10);
        assert_eq!(vm.registers[2], 100);
    }

    #[test]
    fn test_sub() {
        let mut vm = VM::new();
        // LOAD $0 100 -> [1, 0, 0, 100]
        // LOAD $1 10 -> [1, 1, 0, 10]
        // SUB $0 $1 $2 -> [4, 0, 1, 2]
        vm.program = vec![1, 0, 0, 100, 1, 1, 0, 10, 4, 0, 1, 2];
        vm.run();
        assert_eq!(vm.registers[0], 100);
        assert_eq!(vm.registers[1], 10);
        assert_eq!(vm.registers[2], 90);
    }

    #[test]
    fn test_div() {
        let mut vm = VM::new();
        // LOAD $0 21 -> [1, 0, 0, 21]
        // LOAD $1 10 -> [1, 1, 0, 10]
        // DIV $0 $1 $2 -> [5, 0, 1, 2]
        vm.program = vec![1, 0, 0, 21, 1, 1, 0, 10, 5, 0, 1, 2];
        vm.run();
        assert_eq!(vm.registers[0], 21);
        assert_eq!(vm.registers[1], 10);
        assert_eq!(vm.registers[2], 2);
        assert_eq!(vm.remainder, 1);
    }

    #[test]
    fn test_jmp() {
        let mut vm = VM::new();
        vm.registers[0] = 1;
        vm.program = vec![6, 0, 0, 0];
        vm.run_once();
        assert_eq!(vm.pc, 1);
    }

    #[test]
    fn test_jmpf() {
        let mut vm = VM::new();
        vm.registers[0] = 2;
        // JMPF $0
        // 0, 0
        // JMP $0
        vm.program = vec![7, 0, 0, 0, 6, 0, 0, 0];
        vm.run_once();
        assert_eq!(vm.pc, 4);
    }

    #[test]
    fn test_jmpb() {
        let mut vm = VM::new();
        vm.registers[0] = 4;
        vm.registers[1] = 2;
        // JMPB $0
        // 0, 0
        // JMPB $0
        //
        //  This is practically a loop {} given that JMPB is 2 bytes and we are asking it to go
        //  back 2-bytes.
        vm.program = vec![6, 0, 0, 0, 8, 1, 0, 0];
        vm.run_once();
        assert_eq!(vm.pc, 4);
    }

    #[test]
    fn test_illegal_opcode() {
        let mut vm = VM::new();
        vm.program = vec![255];
        vm.run();
        assert_eq!(vm.pc, 1);
    }

    #[test]
    fn test_eq() {
        let mut vm = VM::new();
        vm.registers[0] = 99;
        vm.registers[1] = 99;
        // EQ $0 $1
        // EQ $0 $1
        vm.program = vec![9, 0, 1, 0, 9, 0, 1, 0];
        assert_eq!(false, vm.equal_flag);
        vm.run_once();
        assert_eq!(true, vm.equal_flag);

        vm.registers[1] = 10;
        vm.run_once();
        assert_eq!(false, vm.equal_flag);
    }

    #[test]
    fn test_neq() {
        let mut vm = VM::new();
        vm.registers[0] = 99;
        vm.registers[1] = 99;
        // NEQ $0 $1
        // NEQ $0 $1
        vm.program = vec![10, 0, 1, 0, 10, 0, 1, 0];
        vm.run_once();
        assert_eq!(false, vm.equal_flag);

        vm.registers[1] = 10;
        vm.run_once();
        assert_eq!(true, vm.equal_flag);
    }

    #[test]
    fn test_gt() {
        let mut vm = VM::new();
        vm.registers[0] = 100;
        vm.registers[1] = 99;
        // GT $0 $1
        // GT $0 $1
        vm.program = vec![11, 0, 1, 0, 11, 0, 1, 0];
        vm.run_once();
        assert_eq!(true, vm.equal_flag);

        vm.registers[0] = 10;
        vm.run_once();
        assert_eq!(false, vm.equal_flag);
    }

    #[test]
    fn test_gte() {
        let mut vm = VM::new();
        vm.registers[0] = 100;
        vm.registers[1] = 99;
        // GTE $0 $1
        // GTE $0 $1
        // GTE $0 $1
        vm.program = vec![12, 0, 1, 0, 12, 0, 1, 0, 12, 0, 1, 0];
        vm.run_once();
        assert_eq!(true, vm.equal_flag);

        vm.registers[0] = 99;
        vm.run_once();
        assert_eq!(true, vm.equal_flag);

        vm.registers[0] = 9;
        vm.run_once();
        assert_eq!(false, vm.equal_flag);
    }

    #[test]
    fn test_lt() {
        let mut vm = VM::new();
        vm.registers[0] = 10;
        vm.registers[1] = 99;
        // LT $0 $1
        // LT $0 $1
        vm.program = vec![13, 0, 1, 0, 13, 0, 1, 0];
        vm.run_once();
        assert_eq!(true, vm.equal_flag);

        vm.registers[0] = 100;
        vm.run_once();
        assert_eq!(false, vm.equal_flag);
    }

    #[test]
    fn test_lte() {
        let mut vm = VM::new();
        vm.registers[0] = 100;
        vm.registers[1] = 99;
        // LTE $0 $1
        // LTE $0 $1
        // LTE $0 $1
        vm.program = vec![14, 0, 1, 0, 14, 0, 1, 0, 14, 0, 1, 0];
        vm.run_once();
        assert_eq!(false, vm.equal_flag);

        vm.registers[0] = 99;
        vm.run_once();
        assert_eq!(true, vm.equal_flag);

        vm.registers[1] = 199;
        vm.run_once();
        assert_eq!(true, vm.equal_flag);
    }

    #[test]
    fn test_jeq() {
        let mut vm = VM::new();
        vm.registers[0] = 5;
        vm.equal_flag = true;
        vm.program = vec![15, 0, 0, 0, 1, 2, 3, 4];
        vm.run_once();
        assert_eq!(5, vm.pc);
    }
}
