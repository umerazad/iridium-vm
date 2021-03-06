extern crate num;
#[macro_use]
extern crate num_derive;
#[macro_use]
extern crate log;
extern crate env_logger;

pub mod assembler;
pub mod opcode;
pub mod repl;
pub mod vm;

use repl::REPL;
use structopt::StructOpt;

/// REPL for Iridium VM.
#[derive(StructOpt, Debug)]
struct Opt {}

fn main() {
    env_logger::init();

    let _ = Opt::from_args();

    // REPL takes care of Ctrl-C/D stuff.
    let mut repl = REPL::new();
    repl.run();
}
