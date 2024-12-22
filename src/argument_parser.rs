use arguments::{Args, Commands};
use clap::Parser;

pub mod arguments;

pub fn execute() {
    let args = Args::parse();
    match args.commands {
        Commands::Show => println!("Hello"),
        Commands::Add => println!("World"),
        Commands::Remove => println!("!"),
        Commands::CheckOff => println!("?"),
    }
}
