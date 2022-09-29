mod task;
mod time_utils;
mod scheduler;
mod cli;
mod error;

use scheduler::run_scheduler;
use cli::Args;
use clap::Parser;

fn main() {
    let args = Args::parse();
    let cwd = match args.process() {
        Ok(v) => v,
        Err(e) => panic!("{}", e),
    };

    match run_scheduler(&cwd) {
        Ok(v) => v,
        Err(e) => eprintln!("{}", e),
    }
}
