use std::error::Error;

use crate::database::models::todos;

pub fn execute(todo_id: i32) -> Result<(), Box<dyn Error>> {
    let todo = todos::get_todo_by_id(todo_id)?;
    todos::update_todo(
        todo_id,
        todos::UpdateTodo {
            description: todo.description,
            is_done: true,
        },
    )
}
