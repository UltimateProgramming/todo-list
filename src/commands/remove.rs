use std::error::Error;

use crate::database::models::todos;

pub fn execute(todo_id: i32) -> Result<(), Box<dyn Error>> {
    todos::remove_todo(todo_id)
}
