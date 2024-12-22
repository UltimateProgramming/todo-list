use database::migration_service;
use std::error::Error;

mod argument_parser;
mod commands;
mod database;
mod directory_manager;
mod printer;
mod schema;

fn main() -> Result<(), Box<dyn Error + Send + Sync>> {
    migration_service::execute()?;
    argument_parser::parse();
    Ok(())
}
