use crate::utils::generate_uuid;
use std::str::FromStr;
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

#[derive(Debug, Clone, clap::ValueEnum, serde::Serialize, serde::Deserialize, PartialEq)]
#[clap(rename_all = "lower")]
pub enum TaskStatus {
    Todo,
    InProgress,
    Blocked,
    Done,
}

#[derive(Debug)]
pub struct ParseTaskStatusError;

impl FromStr for TaskStatus {
    type Err = ParseTaskStatusError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "1" => Ok(TaskStatus::Todo),
            "2" => Ok(TaskStatus::InProgress),
            "3" => Ok(TaskStatus::Blocked),
            "4" => Ok(TaskStatus::Done),
            _ => Err(ParseTaskStatusError),
        }
    }
}
