use core::fmt;

use serde::{Deserialize, Serialize};

#[derive(Deserialize, Debug, Serialize, Clone)]
pub enum TodoStatus {
    PENDING,
    COMPLETED,
}

impl TryFrom<String> for TodoStatus {
    type Error = String;
    fn try_from(value: String) -> Result<Self, Self::Error> {
        match value.to_lowercase().as_str() {
            "pending" => Ok(Self::PENDING),
            "completed" => Ok(Self::COMPLETED),
            _ => Err(format!(
                "TodoStatus doesn't have a value that matches '{}'",
                value
            )),
        }
    }
}

// impl TryInto<String> for TodoStatus {
//     type Error = String;
//     fn try_into(self) -> Result<String, Self::Error> {
//         match self {
//             Self::PENDING => Ok("pending".to_string()),
//             Self::COMPLETED => Ok("completed".to_string()),
//         }
//     }
// }

impl fmt::Display for TodoStatus {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Self::COMPLETED => "completed",
                Self::PENDING => "pending",
            }
        )
    }
}

impl PartialEq for TodoStatus {
    fn eq(&self, other: &Self) -> bool {
        self.to_string() == other.to_string()
    }
}

#[derive(Deserialize, Debug, Serialize, Clone)]
pub struct TodoItem {
    pub id: u32,
    pub title: Option<String>,
    pub body: String,
    pub status: TodoStatus,
    pub created_at: String,
    pub updated_at: String,
}
