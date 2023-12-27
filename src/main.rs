use clap::Parser;
use transfer::{handle_copy, handle_move, structs::Cli};

fn main() {
    let args = Cli::parse();
    match args.copy {
        true => handle_copy(args),
        false => handle_move(args),
    }
}
