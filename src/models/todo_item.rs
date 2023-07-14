use serde::{Deserialize, Serialize};

#[derive(Deserialize, Debug, Serialize, Clone)]
pub enum TodoStatus {
    PENDING,
    COMPLETED
}

impl TryFrom<String> for TodoStatus {
    type Error = String;
    fn try_from(value: String) -> Result<Self, Self::Error> {
        match value.to_lowercase().as_str() {
            "pending" => Ok(Self::PENDING),
            "completed" => Ok(Self::COMPLETED),
            _ => Err(format!("TodoStatus doesn't have a value that matches '{}'", value))
        }
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