use crate::database::models::todos::Todo;

pub fn print(todo_list: Vec<Todo>) {
    println!("id\tdescription\tis_done");
    for todo in todo_list {
        println!("{}\t{}\t\t{}", todo.id, todo.description, todo.is_done);
    }
}
