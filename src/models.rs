use crate::cli::TaskStatus;
use crate::utils::generate_uuid; // Import from utils module
use serde::{Deserialize, Serialize};
use uuid::Uuid; // Import TaskStatus from cli module

#[derive(Debug, Serialize, Deserialize)]
pub struct Task {
    pub id: Uuid,
    pub status: TaskStatus,
    pub description: String,
}

impl Task {
    pub fn new(status: TaskStatus, description: String) -> Self {
        Self {
            id: generate_uuid(),
            status,
            description,
        }
    }
}
