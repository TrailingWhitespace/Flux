use crate::errors;
use serde::{Deserialize, Serialize};
use turso::Row;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Todo {
    pub id: String,
    pub title: String,
    pub description: Option<String>,
    pub completed: bool,
    pub completed_at: Option<i64>,
    pub priority: Option<i32>,
    pub due_date: Option<i64>,
    pub position: Option<i32>,
    pub created_at: i64,
    pub deleted_at: Option<i64>,
}

#[derive(Debug, Deserialize)]
pub struct TodoInput {
    pub title: String,
    pub description: Option<String>,
    pub priority: Option<i32>,
    #[serde(rename = "dueDate")] // if frontend sends it like that
    pub due_date: Option<i64>,
}

impl TryFrom<&Row> for Todo {
    type Error = errors::FluxError;

    fn try_from(row: &Row) -> Result<Self, Self::Error> {
        Ok(Todo {
            id: row
                .get_value(0)?
                .as_text()
                .ok_or(errors::FluxError::InvalidColumn("id"))?
                .clone(),
            title: row
                .get_value(1)?
                .as_text()
                .ok_or(errors::FluxError::InvalidColumn("title"))?
                .clone(),
            description: row.get_value(2)?.as_text().cloned(),
            completed: row.get_value(3)?.as_integer().copied().unwrap_or(0) != 0,
            completed_at: row.get_value(4)?.as_integer().copied(),
            priority: row.get_value(5)?.as_integer().map(|v| *v as i32),
            due_date: row.get_value(6)?.as_integer().copied(),
            position: row.get_value(7)?.as_integer().map(|v| *v as i32),
            created_at: row
                .get_value(8)?
                .as_integer()
                .copied()
                .ok_or(errors::FluxError::InvalidColumn("created_at"))?,
            deleted_at: row.get_value(9)?.as_integer().copied(),
        })
    }
}
