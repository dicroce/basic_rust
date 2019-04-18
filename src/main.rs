
// The goal of this project is to provide an introductory tutorial to Rust via super simple examples.
// The main() function below serves as the entry point for both rust binaries and this tutorial. Each
// line in main() calls a run() function found in the file indicated by the prefix (example, look in
// print.rs for the implementation of print::run()). Read each run() function in the order called in
// this tutorial.

mod print;
mod vars;
mod types;
mod strings;
mod tuples;
mod arrays;
mod vectors;
mod conditionals;
mod loops;
mod functions;
mod pointer_ref;
mod structs;
mod enums;
mod cli;

// Rust's entry point function is called main().
fn main() {
    print::run();
    vars::run();
    types::run();
    strings::run();
    tuples::run();
    arrays::run();
    vectors::run();
    conditionals::run();
    loops::run();
    functions::run();
    pointer_ref::run();
    structs::run();
    enums::run();
    cli::run();
}
