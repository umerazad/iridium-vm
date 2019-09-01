pub mod instruction;
pub mod repl;
pub mod vm;

use repl::REPL;
use structopt::StructOpt;

/// REPL for Iridium VM.
#[derive(StructOpt, Debug)]
struct Opt {}

fn main() {
    let _ = Opt::from_args();

    // REPL takes care of Ctrl-C/D stuff.
    let mut repl = REPL::new();
    repl.run();
}
