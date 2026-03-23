use chrono::{DateTime, Utc};
use serde::{Serialize, Deserialize};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Task {
    pub id: u32,
    pub description: String,
    pub status: String,      
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

impl Task {
    pub fn new(id: u32, description: String) -> Self {
        let now = Utc::now();
        Task {
            id,
            description,
            status: "Pendiente".to_string(),
            created_at: now,
            updated_at: now,
        }
    }

    pub fn update_description(&mut self, new_description: String) {
        self.description = new_description;
        self.updated_at = Utc::now();
    }

    pub fn set_status(&mut self, status: &str) {
        self.status = status.to_string();
        self.updated_at = Utc::now();
    }
}