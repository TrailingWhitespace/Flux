use serde::{Deserialize, Serialize};

#[allow(dead_code)]
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Todo {
    pub id: i64,
    pub todo: String,
    pub completed: bool,
    pub completed_at: i64,
}
