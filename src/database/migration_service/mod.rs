use diesel_migrations::{embed_migrations, EmbeddedMigrations, MigrationHarness};
use std::error::Error;

use super::connector;

pub const MIGRATIONS: EmbeddedMigrations = embed_migrations!();

pub fn execute() -> Result<(), Box<dyn Error + Send + Sync>> {
    let mut connection = connector::create_connection();
    connection.run_pending_migrations(MIGRATIONS)?;
    Ok(())
}
