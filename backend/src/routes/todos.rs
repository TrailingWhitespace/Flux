use axum::{
    Json, Router,
    extract::{Path, State},
    routing::{get, post},
};
use chrono::Utc;
use serde::Deserialize;
use turso::{Connection, Rows};

use crate::{errors::FluxError, models::Todo};

pub fn todos_router() -> Router<Connection> {
    Router::new()
        .route("/", get(fetch_todos))
        .route("/insert_todo", post(insert_todo))
        .route("/delete_todo", post(delete_todo))
        .route("/{id}/toggle", post(toggle_todo)) // params instead of body
        .route("/update_todo", post(update_todo))
}

pub async fn fetch_todos(State(conn): State<Connection>) -> Result<Json<Vec<Todo>>, FluxError> {
    let mut rows = conn.query("SELECT * FROM todos", ()).await?;

    let mut todos: Vec<Todo> = vec![];

    while let Some(row) = rows.next().await? {
        let todo = Todo::try_from(&row)?;
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
            (payload.todo, 0), // using a tuple because the values are of different data types (string, i64) so an array cant be used here
        )
        .await?;

    if let Some(row) = rows.next().await? {
        let todo = Todo::try_from(&row)?;
        // println!("{:?}", todo);
        Ok(Json(todo)) // return only id or the entire Todo?
    } else {
        Err(FluxError::CustomError(String::from(
            "Failed to insert todo.",
        )))
    }
}

pub async fn delete_todo(
    State(conn): State<Connection>,
    Json(payload): Json<DeleteTodoRequest>,
) -> Result<Json<Todo>, FluxError> {
    let mut rows: Rows = conn
        .query("DELETE FROM todos WHERE id = ? RETURNING *;", [payload.id])
        .await?;

    if let Some(row) = rows.next().await? {
        let todo = Todo::try_from(&row)?;
        Ok(Json(todo))
    } else {
        Err(FluxError::CustomError(String::from(
            "Failed to delete todo.",
        )))
    }
}

pub async fn toggle_todo(
    State(conn): State<Connection>,
    Path(id): Path<i64>,
) -> Result<Json<Todo>, FluxError> {
    let mut rows: Rows = conn
        .query(
            "
    UPDATE todos
    SET
      completed = 1 - completed,
      completedAt = CASE
        WHEN completed = 0 THEN ?
        ELSE NULL
      END
    WHERE id = ?
    RETURNING *;
  ",
            [Utc::now().timestamp_millis(), id],
        )
        .await?;

    if let Some(row) = rows.next().await? {
        let todo = Todo::try_from(&row)?;
        Ok(Json(todo))
    } else {
        Err(FluxError::NotFound)
    }
}

pub async fn update_todo(
    State(conn): State<Connection>,
    Json(payload): Json<UpdateTodoRequest>,
) -> Result<Json<Todo>, FluxError> {
    let mut rows: Rows = conn
        .query(
            "UPDATE todos SET todo = ? WHERE id = ? RETURNING *;",
            (payload.todo, payload.id),
        )
        .await?;

    if let Some(row) = rows.next().await? {
        let todo = Todo::try_from(&row)?;
        Ok(Json(todo))
    } else {
        Err(FluxError::NotFound)
    }
}

// Validator to check lengths and such?
#[derive(Debug, Deserialize)]
pub struct CreateTodoRequest {
    pub todo: String,
}

#[derive(Debug, Deserialize)]
pub struct UpdateTodoRequest {
    pub id: i64,
    pub todo: String,
}

#[derive(Debug, Deserialize)]
pub struct IdRequest {
    pub id: i64,
}

pub type DeleteTodoRequest = IdRequest;
// pub type ToggleTodoRequest = IdRequest;
