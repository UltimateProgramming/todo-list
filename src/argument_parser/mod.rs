use arguments::{Args, Commands};
use clap::Parser;

use crate::commands::{add, check_off, remove, show};

pub mod arguments;

pub fn parse() {
    let args = Args::parse();
    let _ = match args.commands {
        Commands::Show => show::execute(),
        Commands::Add { description } => add::execute(description),

        Commands::Remove { todo_id } => remove::execute(todo_id),
        Commands::CheckOff { todo_id } => check_off::execute(todo_id),
    };
}
