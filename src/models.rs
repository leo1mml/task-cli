use crate::utils::generate_uuid;
use strum::EnumString;
use uuid::Uuid; // Import TaskStatus from cli module

#[derive(Debug, serde::Serialize, serde::Deserialize)]
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

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize, PartialEq, EnumString)]
#[strum(ascii_case_insensitive)]
pub enum TaskStatus {
    Todo,
    InProgress,
    Blocked,
    Done,
    Custom { text: String },
}
