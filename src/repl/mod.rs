use crate::assembler::parsers::parse_program;
use crate::assembler::Assembler;
use crate::vm::VM;
use std;
use std::fs;
use std::io::{self, Write};
use std::process;

use rustyline::error::ReadlineError;
use rustyline::{CompletionType, Config, Editor};

#[cfg(unix)]
static PROMPT: &str = "\x1b[1;32miridium >>\x1b[0m ";

#[cfg(windows)]
static PROMPT: &str = "iridium >> ";

/// Key structure for the Assembly REPL.
pub struct REPL {
    // VM instance that executes the assembly.
    vm: VM,

    // Assembler
    asm: Assembler,
}

impl REPL {
    /// Create a new REPL instance.
    pub fn new() -> Self {
        REPL {
            vm: VM::new(),
            asm: Assembler::new(),
        }
    }

    /// Execute REPL loop.
    pub fn run(&mut self) {
        let config = Config::builder()
            .history_ignore_space(true)
            .completion_type(CompletionType::List)
            .build();

        let mut rl = Editor::<()>::with_config(config);

        if rl.load_history("history.txt").is_ok() {
            println!("Loaded history.");
        }

        println!();
        println!("Welcome to Iridium VM!");
        println!(
            "Current working directory: {}",
            std::env::current_dir().unwrap().display()
        );
        println!("Press Ctrl-D or enter \"q\" to exit.");
        println!();

        loop {
            let readline = rl.readline(PROMPT);

            match readline {
                Ok(line) => {
                    // Update history.
                    rl.add_history_entry(line.as_str());
                    match line.as_str() {
                        ".reset" => {
                            self.vm = VM::new();
                            println!("Resetting VM state. Everything should be clean now.");
                        }
                        ".q" | ".quit" => {
                            println!("Goodbye!");
                            process::exit(0);
                        }
                        ".hs" | ".history" => {
                            for cmd in rl.history().iter() {
                                println!("{}", cmd);
                            }
                        }
                        ".regs" | ".registers" => {
                            self.dump_registers();
                        }
                        ".vm" => {
                            self.vm.dump_state();
                        }
                        ".load" => {
                            self.load_file();
                        }
                        ".n" | ".next" => {
                            self.vm.run_once();
                        }
                        ".g" | ".go" => {
                            self.vm.run();
                        }
                        ".h" | ".help" => {
                            self.print_help();
                        }
                        inst => {
                            if inst.starts_with(".") {
                                println!("Unrecognized instruction. Use .help for detailed help.");
                            } else {
                                let bytecode = self
                                    .asm
                                    .assemble(line.as_str())
                                    .expect("Failed to parse program.");
                                self.vm.add_bytes(&bytecode);
                                self.vm.run_once();
                            }
                        }
                    }
                }
                Err(ReadlineError::Interrupted) => {
                    println!("Ctrl-C");
                    break;
                }
                Err(ReadlineError::Eof) => {
                    println!("Ctrl-D");
                    break;
                }
                Err(err) => {
                    println!("Error: {:?}", err);
                    break;
                }
            }
        }
    }

    fn print_help(&self) {
        println!("Command:  Description\n-------  ------------");
        println!(".reset    Reset the VM state.");
        println!(".history  See the command history.");
        println!(".regs     Dump registers.");
        println!(".vm       Dump VM state excluding registers.");
        println!(".load     Load an assembly file. It prompts for the file path.");
        println!(".n        Execute next instruction.");
        println!(".go       Execute rest of the program.");
        println!(".help     Print this help message.");
        println!(".quit     Quit the REPL. You can also use Ctrl-C or Ctrl-D.");
    }

    fn load_file(&mut self) {
        print!("Please enter file path: ");
        // stdout is line-buffered and print! doesn't flush.
        io::stdout().flush().expect("Failed to flush stdout.");

        let mut file = String::new();
        io::stdin()
            .read_line(&mut file)
            .expect("Failed to read file name.");

        // read_line includes the ending newline character.
        let file = file.trim();
        let contents = fs::read_to_string(file).expect("Failed to read file.");

        let bytecode = self
            .asm
            .assemble(&contents)
            .expect("Failed to assemble program.");
        self.vm.add_bytes(&bytecode);
    }

    fn dump_registers(&self) {
        println!("Registers:\n----------");
        for (i, r) in self.vm.registers().enumerate() {
            println!("${}: {}", i, r);
        }
    }
}
