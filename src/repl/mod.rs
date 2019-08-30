use crate::vm::VM;
use std;
use std::io::{self, Write};
use std::num::ParseIntError;
use std::process;

/// Key structure for the Assembly REPL.
pub struct REPL {
    // Buffer to hold assembly commands.
    command_buffer: Vec<String>,

    // VM instance that executes the assembly.
    vm: VM,
}

impl REPL {
    /// Create a new REPL instance.
    pub fn new() -> Self {
        REPL {
            command_buffer: vec![],
            vm: VM::new(),
        }
    }

    /// Execute REPL loop.
    pub fn run(&mut self) {
        println!("Welcome to Iridium VM! Type away!");
        loop {
            print!(">>> ");
            // stdout is line-buffered by default that's why we need to explicitly flush it to
            // ensure that the prompt actually prints.
            io::stdout().flush().expect("Unable to flush output.");

            let mut buffer = String::new();

            // TODO: Cleanup expect once basic testing is done.
            io::stdin()
                .read_line(&mut buffer)
                .expect("Unable to read line");

            let buffer = buffer.trim();
            self.command_buffer.push(buffer.to_string());

            match buffer {
                "q" | "quit" => {
                    println!("Goodbye!");
                    process::exit(0);
                }
                "hs" | "history" => {
                    for cmd in &self.command_buffer {
                        println!("{}", cmd);
                    }
                }
                "r" | "registers" => {
                    self.dump_registers();
                }
                _ => {
                    // Let's try and interpret the input as hex values and see if it
                    // makes sense.
                    match self.parse_hex(buffer) {
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
