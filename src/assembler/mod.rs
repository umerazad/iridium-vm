use std::collections::HashMap;

/// This module contains implementation of our simple two-pass assembler
/// for the Iridium VM.
pub mod assembly_instruction;
pub mod parsers;
pub mod program;
pub mod token;

use crate::vm::VM;
use program::Program;

#[derive(Debug)]
pub enum SymbolType {
    Label,
}

#[derive(Debug)]
pub struct SymbolInfo {
    offset: u32,
    symbol_type: SymbolType,
}

impl SymbolInfo {
    fn new(offset: u32, t: SymbolType) -> Self {
        SymbolInfo {
            offset,
            symbol_type: t,
        }
    }
}

pub type SymbolTable = HashMap<String, SymbolInfo>;

#[derive(Debug, Clone)]
pub enum AssemblerPass {
    // In the first pass, we just collect all the symbols/labels and their
    // locations.
    First,

    // In the second path, we general final byte-code and also patch up addresses
    // for any forward/backward jumps and other symbols.
    Second,
}

#[derive(Debug)]
pub struct Assembler {
    pass: AssemblerPass,
    symbol_table: SymbolTable,
}

impl Assembler {
    /// Creates a new Assembler instance.
    pub fn new() -> Assembler {
        Assembler {
            pass: AssemblerPass::First,
            symbol_table: SymbolTable::new(),
        }
    }

    /// Assembles the specified program.
    pub fn assemble(&mut self, prog: &str) -> Option<Vec<u8>> {
        match parsers::parse_program(prog) {
            Ok((_leftover, program)) => {
                self.run_pass1(&program);
                Some(self.run_pass2(&program))
            }
            Err(e) => {
                eprintln!("Failed to assemble program. Error: {:?}", e);
                None
            }
        }
    }

    // Runs first pass of the assembler. Here we basically just build the
    // symbol table for all the labels and record their offsets.
    fn run_pass1(&mut self, prog: &Program) {
        // program counter.
        let mut pc = 0;

        // Record addresses of all labels in the symbol table.
        for i in &prog.instructions {
            if i.has_label() {
                match i.get_label() {
                    Some(name) => {
                        let info = SymbolInfo::new(pc, SymbolType::Label);
                        self.symbol_table.insert(name, info);
                    }
                    None => (),
                }
            }

            pc += assembly_instruction::INSTRUCTION_SIZE;
        }

        // We are ready to move to next pass.
        self.pass = AssemblerPass::Second;
    }

    // Run second pass where we generate complete byte-code.
    fn run_pass2(&mut self, prog: &Program) -> Vec<u8> {
        prog.to_bytes(&self.symbol_table)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_assemble() {
        let mut assembler = Assembler::new();

        let prog_string = r##" load $0 #20
                 load $1 #30
                 add $0 $1 $2
                 hlt"##;

        let program = assembler.assemble(prog_string).unwrap();
        let mut vm = VM::new();
        vm.add_bytes(&program);
        vm.run();
        assert_eq!(vm.register(0), 20);
        assert_eq!(vm.register(1), 30);
        assert_eq!(vm.register(2), 50);
    }
}
