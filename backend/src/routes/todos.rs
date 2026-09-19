use axum::{
    Json, Router,
    extract::{Path, Query, State},
    routing::{delete, get, post, put},
};
use chrono::Utc;
use serde::Deserialize;
use turso::{Connection, Rows, Value, params_from_iter};
use uuid::Uuid;

use crate::{
    errors::FluxError,
    models::todos::{Todo, TodoInput},
};

pub fn todos_router() -> Router<Connection> {
    // fetch a single todo route?
    Router::new()
        .route("/", get(fetch_todos))
        .route("/insert_todo", post(insert_todo))
        .route("/{id}/delete_todo", delete(delete_todo))
        .route("/{id}/toggle", post(toggle_todo)) // params instead of body
        .route("/{id}/update_todo", put(update_todo))
        .route("/{id}/restore_todo", post(restore_todo))
    // do this like
    // .route("/todos",put(update_todo).delete(delete_todo)) ?
}

pub async fn fetch_todos(
    State(conn): State<Connection>,
    Query(params): Query<TodoQueryParams>,
) -> Result<Json<Vec<Todo>>, FluxError> {
    let mut sql = String::from("SELECT * FROM todos WHERE deleted_at IS NULL"); // exclude deleted todos by checking if deleted_at is NULL
    let mut args: Vec<Value> = Vec::new();

    // Completed query
    if let Some(completed) = params.completed {
        sql.push_str(" AND completed = ?");
        args.push(Value::Integer(completed as i64));
    }

    sql.push_str(" ORDER BY position ASC"); // send ordered todos using position

    let mut rows: Rows = conn.query(&sql, params_from_iter(args)).await?;

    let mut todos: Vec<Todo> = Vec::new();

    while let Some(row) = rows.next().await? {
        let todo = Todo::try_from(&row)?;
        todos.push(todo);
    }

    Ok(Json(todos))
}

pub async fn insert_todo(
    State(conn): State<Connection>,
    Json(payload): Json<TodoInput>, // insert and update now use the TodoInput model instead of Request structs
) -> Result<Json<Todo>, FluxError> {
    if payload.title.trim().is_empty() {
        return Err(FluxError::CustomError("Todo cannot be empty".to_string()));
    }

    let id = Uuid::new_v4().to_string();

    let mut rows: Rows = conn
        .query(
            "INSERT INTO todos (id, title, description, completed, priority, due_date, created_at)
             VALUES (?, ?, ?, ?, ?, ?, ?) RETURNING *;",
            (
                id,
                payload.title,
                payload.description,
                0,
                payload.priority,
                payload.due_date,
                Utc::now().timestamp_millis(),
            ), // using a tuple because the values are of different data types (string, i64) so an array cant be used here
        )
        .await?;

    if let Some(row) = rows.next().await? {
        Ok(Json(Todo::try_from(&row)?))
    } else {
        Err(FluxError::CustomError(String::from(
            "Failed to insert todo.",
        )))
    }
}

pub async fn delete_todo(
    State(conn): State<Connection>,
    Path(id): Path<String>,
) -> Result<Json<Todo>, FluxError> {
    let mut rows: Rows = conn
        .query(
            "UPDATE todos SET deleted_at = ? WHERE id = ? AND deleted_at IS NULL RETURNING *;",
            (Utc::now().timestamp_millis(), id),
        )
        .await?;
    // Soft delete todos, only give the todo a deleted_at timestamp and maybe purge them in batches later,
    // either periodically or randomly or having a trashed todos view on the frontend which you can empty
    // or using some other condition

    if let Some(row) = rows.next().await? {
        let todo = Todo::try_from(&row)?;
        Ok(Json(todo))
    } else {
        Err(FluxError::NotFound)
    }
}

pub async fn toggle_todo(
    State(conn): State<Connection>,
    Path(id): Path<String>,
) -> Result<Json<Todo>, FluxError> {
    let mut rows: Rows = conn
        .query(
            "
    UPDATE todos
    SET
      completed = 1 - completed,
      completed_at = CASE
        WHEN completed = 0 THEN ?
        ELSE NULL
      END
    WHERE id = ?
    RETURNING *;
  ",
            (Utc::now().timestamp_millis(), id),
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
    Path(id): Path<String>,
    Json(payload): Json<TodoInput>,
) -> Result<Json<Todo>, FluxError> {
    let mut rows: Rows = conn
        .query(
            "UPDATE todos SET title = ?, description = ?, priority = ?, due_date = ? WHERE id = ? RETURNING *;",
            (payload.title, payload.description, payload.priority, payload.due_date, id),
        )
        .await?;

    if let Some(row) = rows.next().await? {
        Ok(Json(Todo::try_from(&row)?))
    } else {
        Err(FluxError::NotFound)
    }
}

pub async fn restore_todo(
    State(conn): State<Connection>,
    Path(id): Path<String>,
) -> Result<Json<Todo>, FluxError> {
    let mut rows: Rows = conn
        .query(
            "UPDATE todos SET deleted_at = NULL WHERE id = ? AND deleted_at IS NOT NULL RETURNING *;",
            [id],
        )
        .await?;
    // just setting deleted_at back to NULL so the todo can be sent along normally on the next request

    if let Some(row) = rows.next().await? {
        let todo = Todo::try_from(&row)?;
        Ok(Json(todo))
    } else {
        Err(FluxError::NotFound)
    }
}

// TODO: Periodic or delete all todos whose "deleted_at" is NOT NULL, at once
#[allow(dead_code)]
pub async fn purge_deleted_todos() {}

#[derive(Debug, Deserialize)]
pub struct TodoQueryParams {
    completed: Option<bool>,
}

// TODO: make this a route and then the frontend optimistically updates the order on drag and drop and then sends the new order (a list of ids)
// which get enumerated here and the position in the db gets updated
// Orrrrrrrrrrrr do i not want a drag order list at all and just order it by priority on the frontend?
// or
// keep this route to reorder the todos
// and in the frontend i can have a sort button which can sort in different ways like
// position - if theres a drag order mechanism
// priority - with tags or colors
// due date - closest due date appears on top
// created at - latest on top
// all modify position in the db at the end
// then maybe even a flip button to see it in reverse

// i also need to use this for making completed todos go to the top of the completed list when i make a collapsible section?
// or is the pending and completed tags thing fine? (it is)
#[allow(dead_code)]
#[derive(Deserialize)]
pub struct ReorderRequest {
    pub ids: Vec<String>, // full ordered list of todo ids after drag
}

#[allow(dead_code)]
pub async fn reorder_todos(
    State(conn): State<Connection>,
    Json(payload): Json<ReorderRequest>,
) -> Result<Json<()>, FluxError> {
    for (i, id) in payload.ids.iter().enumerate() {
        conn.execute(
            "UPDATE todos SET position = ? WHERE id = ?",
            (i as i64, id.clone()),
        )
        .await?;
    }
    Ok(Json(()))
}
