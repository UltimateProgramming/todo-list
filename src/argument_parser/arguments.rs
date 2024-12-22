use clap::{Parser, Subcommand};

#[derive(Parser)]
pub struct Args {
    #[command(subcommand)]
    pub commands: Commands,
}

#[derive(Subcommand)]
pub enum Commands {
    Show,
    #[command(arg_required_else_help = true)]
    Add {
        description: String,
    },
    #[command(arg_required_else_help = true)]
    Remove {
        todo_id: i32,
    },
    CheckOff {
        todo_id: i32,
    },
}
