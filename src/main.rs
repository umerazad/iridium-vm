pub mod instruction;
pub mod repl;
pub mod vm;

use ctrlc;
use repl::REPL;
use structopt::StructOpt;

/// REPL for Iridium VM.
#[derive(StructOpt, Debug)]
struct Opt {}

fn main() {
    let _ = Opt::from_args();

    // Gracefully handle the Ctrl-C stuff.
    ctrlc::set_handler(|| {
        println!("\nCtrl+C detected. Terminating.");
        std::process::exit(1);
    })
    .expect("Error setting-up Ctrl-C handler.");

    let mut repl = REPL::new();
    repl.run();
}
