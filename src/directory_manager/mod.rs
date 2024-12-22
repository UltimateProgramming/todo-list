use std::{fs::create_dir, path::PathBuf};

use directories::ProjectDirs;

pub fn get_default_database_path(database_name: &str) -> PathBuf {
    let path = ProjectDirs::from("", "", "todo-list").expect("no directory found");
    let data_dir = path.data_dir();
    if !data_dir.exists() {
        create_dir(data_dir).expect("data directory could not be created");
    }
    path.data_dir().to_path_buf().join(database_name)
}
