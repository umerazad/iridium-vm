pub mod instruction;
pub mod repl;
pub mod vm;

use repl::REPL;

fn main() {
    let mut repl = REPL::new();
    repl.run();
}
