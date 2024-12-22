use crate::directory_manager::get_default_database_path;
use diesel::prelude::*;

const DB_PATH: &str = "todo-list.db";

pub fn create_connection() -> SqliteConnection {
    let database_url = get_default_database_path(DB_PATH);
    let path = database_url.to_str().unwrap();
    SqliteConnection::establish(path).unwrap_or_else(|_| panic!("failed to establish a connection"))
}
