use crate::errors;
use serde::{Deserialize, Serialize};
use turso::Row;

#[allow(dead_code)]
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Todo {
    pub id: i64,
    pub todo: String,
    pub completed: bool,
    pub completed_at: i64,
}

impl TryFrom<&Row> for Todo {
    type Error = errors::FluxError;

    fn try_from(row: &Row) -> Result<Self, Self::Error> {
        Ok(Todo {
            id: row.get_value(0)?.as_integer().unwrap_or(&0).clone(),
            todo: row
                .get_value(1)?
                .as_text()
                .unwrap_or(&String::new())
                .clone(),
            completed: row.get_value(2)?.as_integer().unwrap_or(&0) != &0,
            completed_at: row.get_value(3)?.as_integer().unwrap_or(&0).clone(),
        })
    }
}
