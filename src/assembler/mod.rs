use std::collections::HashMap;

/// This module contains implementation of our simple two-pass assembler
/// for the Iridium VM.
pub mod assembly_instruction;
pub mod parsers;
pub mod program;
pub mod token;

use crate::vm::VM;
use program::Program;

/// Executable header has the following format:
///      |---------------------------------------------------------|
///      | Bytes[0..4] contain the 4 byte magic header. It is set  |
///      |       to AZAD in hex i.e. 41 5A 41 44                   |
///      |---------------------------------------------------------|
///      | Bytes[4] Contains 1 byte version. Its set to 1 for now. |
///      |---------------------------------------------------------|
///      | Remaining 59 bytes are padded with zeros for now.       |
///      |---------------------------------------------------------|

pub const BIN_HEADER_LENGTH: usize = 64;
pub const BIN_HEADER_OFFSET: usize = 0;

pub const BIN_HEADER_PREFIX: [u8; 4] = [0x41, 0x5A, 0x41, 0x44];

pub const BIN_VERSION_OFFSET: usize = 4; // fifth byte.
pub const BIN_VERSION: u8 = 1;

#[derive(Debug)]
pub enum SymbolType {
    Label,
    Integer,
    String,
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

#[derive(Debug, Default)]
pub struct Section {
    start: Option<usize>,
    size: Option<usize>,
}

#[derive(Debug)]
pub enum AssemblerSection {
    /// Code section. Start signifies the start of section
    Code(Section),

    /// Read/write data section for initialized stuff.
    Data(Section),

    Unknown,
}

impl Default for AssemblerSection {
    fn default() -> Self {
        AssemblerSection::Unknown
    }
}

impl<'a> From<&'a str> for AssemblerSection {
    fn from(s: &'a str) -> AssemblerSection {
        match s {
            "code" => AssemblerSection::Code(Section::default()),
            "data" => AssemblerSection::Data(Section::default()),
            _ => AssemblerSection::Unknown,
        }
    }
}

#[derive(Debug)]
pub struct Assembler {
    /// Currently active pass of our two-pass assembler.
    pass: AssemblerPass,

    /// Map of symbols
    symbol_table: SymbolTable,

    /// Read/write data section.
    data: Vec<u8>,

    /// Code section.
    code: Vec<u8>,

    /// List of all sections that we've seen so far. We allow multiple code/data
    /// segments.
    segments: Vec<AssemblerSection>,

    /// Section that we are currently processing.
    current_section: AssemblerSection,

    /// Instruction that assembler is currently converting to bytecode. This is
    /// roughly the line # of the input program and we use it to report
    /// diagnostic messages.
    current_instruction: u32,
}

impl Assembler {
    /// Creates a new Assembler instance.
    pub fn new() -> Assembler {
        Assembler {
            pass: AssemblerPass::First,
            symbol_table: SymbolTable::new(),
            data: vec![],
            code: vec![],
            segments: vec![],
            current_section: AssemblerSection::Unknown,
            current_instruction: 0,
        }
    }

    pub fn generate_header() -> Vec<u8> {
        let mut header = vec![0; BIN_HEADER_LENGTH];

        // Write magic number.
        for (i, v) in BIN_HEADER_PREFIX.into_iter().enumerate() {
            header[i] = *v;
        }
        header[BIN_VERSION_OFFSET] = BIN_VERSION;
        header
    }

    /// Assembles the specified program.
    pub fn assemble(&mut self, prog: &str) -> Option<Vec<u8>> {
        match parsers::parse_program(prog) {
            // TODO: Deal with _leftover. This should be an error if the
            // parser can't fully consume the program.
            Ok((_leftover, program)) => {
                // Generate header.
                let mut executable = Assembler::generate_header();

                // Generate bytecode.
                self.run_pass1(&program);
                let mut bytecode = self.run_pass2(&program);

                // Append the bytecode to the executable.
                executable.append(&mut bytecode);
                Some(executable)
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
