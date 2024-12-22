use std::error::Error;

use crate::{database::models::todos, printer};

pub fn execute() -> Result<(), Box<dyn Error>> {
    let todos = todos::get_todos(1)?;
    printer::print(todos);
    Ok(())
}
