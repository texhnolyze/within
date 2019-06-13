extern crate ansi_term;
extern crate clap;
extern crate rayon;

mod app;
mod runner;

use std::io;
use std::process;

use crate::app::App;
use crate::errors::handle_error;
use crate::runner::Runner;

mod errors {
    use std::io::Error;

    pub fn handle_error(error: Error, message: String) {
        use ansi_term::Colour::Red;
        eprintln!("{}: {}: {}", Red.paint("[within error]"), message, error);
    }
}

fn main() {
    let result = run();

    match result {
        Err(e) => {
            handle_error(e, String::new());
            process::exit(1);
        }
        _ => {
            process::exit(0);
        }
    }
}

fn run() -> io::Result<()> {
    let app = App::new();
    Runner::execute_commands(&app.config())
}
