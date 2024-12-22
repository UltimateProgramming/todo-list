use std::error::Error;

use crate::database::models::todos;

pub fn execute(description: String) -> Result<(), Box<dyn Error>> {
    todos::add_todo(todos::NewTodo::new(description, false))
}
