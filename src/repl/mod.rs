use crate::assembler::parsers::parse_program;
use crate::vm::VM;
use std;
use std::process;

use rustyline::error::ReadlineError;
use rustyline::{CompletionType, Config, Editor};

#[cfg(unix)]
static PROMPT: &'static str = "\x1b[1;32miridium >>\x1b[0m ";

#[cfg(windows)]
static PROMPT: &'static str = "iridium >> ";

/// Key structure for the Assembly REPL.
pub struct REPL {
    // VM instance that executes the assembly.
    vm: VM,
}

impl REPL {
    /// Create a new REPL instance.
    pub fn new() -> Self {
        REPL { vm: VM::new() }
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
        println!("Press Ctrl-D or enter \"q\" to exit.");
        println!();

        loop {
            let readline = rl.readline(PROMPT);

            match readline {
                Ok(line) => {
                    // Update history.
                    rl.add_history_entry(line.as_str());
                    match line.as_str() {
                        "q" | "quit" => {
                            println!("Goodbye!");
                            process::exit(0);
                        }
                        "hs" | "history" => {
                            for cmd in rl.history().iter() {
                                println!("{}", cmd);
                            }
                        }
                        "r" | "registers" => {
                            self.dump_registers();
                        }
                        _ => {
                            let parsed_program = parse_program(line.as_str());
                            if parsed_program.is_err() {
                                println!(
                                    "Unable to parse input. Error: {:?}",
                                    parsed_program.err()
                                );
                                continue;
                            }

                            let (_, result) = parsed_program.unwrap();
                            let bytecode = result.to_bytes();
                            self.vm.add_bytes(&bytecode);
                            // Run stuff.
                            self.vm.run_once();
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

    fn dump_registers(&self) {
        println!("Registers:\n----------");
        for (i, r) in self.vm.registers().enumerate() {
            println!("${}: {}", i, r);
        }
    }
}
