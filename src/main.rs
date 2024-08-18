
use clap::Parser;

mod structs;

fn main() {
    let args = structs::Args::parse();

    eprintln!("args = {:?}", args);

}
