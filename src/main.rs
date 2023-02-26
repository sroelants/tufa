mod otp;
mod db;
mod crypto;
mod cli;
mod util;

use cli::Tufa;
use clap::Parser;

fn main() {
    let tufa = Tufa::parse();

    match tufa.run() {
        Ok(_) => {}
        Err(error) => println!("💥 Error: {}", error),
    }
}
