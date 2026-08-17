use axum::{
    Json, Router,
    extract::State,
    routing::{get, post},
};
use turso::Connection;

use crate::{errors::FluxError, models::Todo};

pub fn todos_router() -> Router<Connection> {
    Router::new()
        .route("/", get(fetch_todos))
        .route("/add_todo", post(add_todo))
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

pub async fn add_todo(State(_conn): State<Connection>) -> FluxError {
    FluxError::NotFound // test by throwing FluxError
}
