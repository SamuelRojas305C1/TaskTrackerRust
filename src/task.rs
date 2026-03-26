use chrono::{DateTime, Utc};
use serde::{Serialize, Deserialize};
use std::fmt;

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum TaskStatus {
    Pendiente,
    EnCurso,
    Hecho,
}

impl fmt::Display for TaskStatus {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            TaskStatus::Pendiente => write!(f, "Pendiente"),
            TaskStatus::EnCurso   => write!(f, "En Curso"),
            TaskStatus::Hecho     => write!(f, "Hecho"),
        }
    }
}

impl TaskStatus {
    /// Parsea un string del usuario de forma case-insensitive
    /// Retorna None si el valor no es válido
    pub fn from_str_ci(s: &str) -> Option<Self> {
        match s.to_lowercase().as_str() {
            "pendiente"   => Some(TaskStatus::Pendiente),
            "en curso" | "en_curso" | "encurso" => Some(TaskStatus::EnCurso),
            "hecho"       => Some(TaskStatus::Hecho),
            _ => None,
        }
    }
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Task {
    pub id: u32,
    pub description: String,
    pub status: TaskStatus,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

impl Task {
    pub fn new(id: u32, description: String) -> Self {
        let now = Utc::now();
        Task {
            id,
            description,
            status: TaskStatus::Pendiente,
            created_at: now,
            updated_at: now,
        }
    }

    pub fn update_description(&mut self, new_description: String) {
        self.description = new_description;
        self.updated_at = Utc::now();
    }

    pub fn set_status(&mut self, status: TaskStatus) {
        self.status = status;
        self.updated_at = Utc::now();
    }
}