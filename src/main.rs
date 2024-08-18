
use clap::Parser;

mod structs;

fn main() {
    let args = structs::Args::parse();

    eprintln!("args = {:?}", args);

    if args.command.as_str() == "server" {
      server(&args);
    }
    else if args.command.as_str() == "client" {
      client(&args);
    }
    else {
      eprintln!("Unknown command {}", args.command);
    }
}

fn client(args: &structs::Args) {

}


fn server(args: &structs::Args) {

}

