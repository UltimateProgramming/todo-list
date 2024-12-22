use std::error::Error;

use diesel::prelude::*;

use crate::database::connector;
use crate::schema::todos::dsl::*;

#[derive(Queryable, Selectable, Default)]
#[diesel(table_name = crate::schema::todos)]
#[diesel(check_for_backend(diesel::sqlite::Sqlite))]
pub struct Todo {
    pub id: i32,
    pub description: String,
    pub is_done: bool,
    pub created_at: chrono::NaiveDateTime,
    pub modified_at: chrono::NaiveDateTime,
}

#[derive(Insertable)]
#[diesel(table_name = crate::schema::todos)]
#[diesel(check_for_backend(diesel::sqlite::Sqlite))]
pub struct NewTodo {
    pub description: String,
    pub is_done: bool,
    pub created_at: chrono::NaiveDateTime,
    pub modified_at: chrono::NaiveDateTime,
}

impl NewTodo {
    pub fn new(descr: String, done: bool) -> Self {
        Self {
            description: descr,
            is_done: done,
            created_at: chrono::Utc::now().naive_utc(),
            modified_at: chrono::Utc::now().naive_utc(),
        }
    }
}

#[derive(AsChangeset)]
#[diesel(table_name = crate::schema::todos)]
#[diesel(check_for_backend(diesel::sqlite::Sqlite))]
pub struct UpdateTodo {
    pub description: String,
    pub is_done: bool,
}

impl UpdateTodo {
    pub fn new(descr: String, done: bool) -> Self {
        Self {
            description: descr,
            is_done: done,
        }
    }
}

pub fn get_todo_by_id(todo_id: i32) -> Result<Todo, Box<dyn Error>> {
    let connection = &mut connector::create_connection();
    let todo = todos
        .filter(id.eq(todo_id))
        .limit(1)
        .select(Todo::as_select())
        .first(connection)?;

    Ok(todo)
}

pub fn get_todos(page: i64) -> Result<Vec<Todo>, Box<dyn Error>> {
    let page_size = 50;
    let connection = &mut connector::create_connection();
    let todo_list: Vec<Todo> = todos
        .limit(page_size)
        .offset((page - 1) * page_size)
        .select(Todo::as_select())
        .load(connection)?;
    Ok(todo_list)
}

pub fn add_todo(todo: NewTodo) -> Result<(), Box<dyn Error>> {
    let todo_list = vec![todo];
    add_todos(todo_list)
}

pub fn add_todos(todo_list: Vec<NewTodo>) -> Result<(), Box<dyn Error>> {
    let connection = &mut connector::create_connection();
    diesel::insert_into(todos)
        .values(todo_list)
        .execute(connection)?;
    Ok(())
}

pub fn remove_todo(todo_id: i32) -> Result<(), Box<dyn Error>> {
    let ids = vec![todo_id];
    remove_todos(ids)
}

pub fn remove_todos(ids: Vec<i32>) -> Result<(), Box<dyn Error>> {
    let connection = &mut connector::create_connection();
    diesel::delete(todos.filter(id.eq_any(ids))).execute(connection)?;

    Ok(())
}

pub fn update_todo(todo_id: i32, todo: UpdateTodo) -> Result<(), Box<dyn Error>> {
    let connection = &mut connector::create_connection();
    diesel::update(todos.filter(id.eq(todo_id)))
        .set(todo)
        .execute(connection)?;
    Ok(())
}
