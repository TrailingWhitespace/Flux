use axum::{
    Json, Router,
    extract::State,
    routing::{get, post},
};
use serde::Deserialize;
use turso::{Connection, Rows};

use crate::{errors::FluxError, models::Todo};

pub fn todos_router() -> Router<Connection> {
    Router::new()
        .route("/", get(fetch_todos))
        .route("/insert_todo", post(insert_todo))
}

pub async fn fetch_todos(State(conn): State<Connection>) -> Result<Json<Vec<Todo>>, FluxError> {
    let mut rows = conn.query("SELECT * FROM todos", ()).await?;

    let mut todos: Vec<Todo> = vec![];

    while let Some(row) = rows.next().await? {
        let todo = Todo {
            id: row.get_value(0)?.as_integer().unwrap_or(&0).clone(),
            todo: row
                .get_value(1)?
                .as_text()
                .unwrap_or(&String::new())
                .clone(),
            completed: row.get_value(2)?.as_integer().unwrap_or(&0) != &0,
            completed_at: row.get_value(3)?.as_integer().unwrap_or(&0).clone(),
        };
        todos.push(todo);
    }

    Ok(Json(todos))
}

pub async fn insert_todo(
    State(conn): State<Connection>,
    Json(payload): Json<CreateTodoRequest>,
) -> Result<Json<Todo>, FluxError> {
    if payload.todo.trim().is_empty() {
        return Err(FluxError::CustomError("Todo cannot be empty".to_string()));
    }

    let mut rows: Rows = conn
        .query(
            "INSERT INTO todos (todo, completed) VALUES (?, ?) RETURNING *;",
            (payload.todo, 0),
        )
        .await?;

    if let Some(row) = rows.next().await? {
        let todo: Todo = Todo {
            id: row.get_value(0)?.as_integer().unwrap_or(&0).clone(),
            todo: row
                .get_value(1)?
                .as_text()
                .unwrap_or(&String::new())
                .clone(),
            completed: row.get_value(2)?.as_integer().unwrap_or(&0) != &0,
            completed_at: row.get_value(3)?.as_integer().unwrap_or(&0).clone(),
        };
        // println!("{:?}", todo);
        Ok(Json(todo)) // return only id or the entire Todo?
    } else {
        Err(FluxError::CustomError(String::from(
            "Failed to insert todo.",
        )))
    }
}

#[derive(Debug, Deserialize)]
pub struct CreateTodoRequest {
    pub todo: String,
}
