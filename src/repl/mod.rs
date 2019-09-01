use crate::vm::VM;
use std;
use std::num::ParseIntError;
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
                            // Let's try and interpret the input as hex values and see if it
                            // makes sense.
                            match self.parse_hex(line.as_str()) {
                                Ok(bytes) => {
                                    for byte in bytes {
                                        self.vm.add_byte(byte);
                                    }
                                }
                                Err(_) => {
                                    println!("Invalid input. For raw bytecode please enter 4 groups of 2 hex chars.");
                                    continue;
                                }
                            }

                            // We land here if the hex parsing was successful. Let's try and execute the
                            // newly added byte-code.
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

    fn parse_hex(&self, bytes: &str) -> Result<Vec<u8>, ParseIntError> {
        let elements = bytes.split(" ").collect::<Vec<&str>>();
        let mut result: Vec<u8> = vec![];

        for s in elements {
            match u8::from_str_radix(&s, 16) {
                Ok(v) => result.push(v),
                Err(e) => return Err(e),
            }
        }
        Ok(result)
    }

    fn dump_registers(&self) {
        println!("Registers:\n----------");
        for (i, r) in self.vm.registers().enumerate() {
            println!("${}: {}", i, r);
        }
    }
}
