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


// Tests unitarios


#[cfg(test)]
mod tests {
    use super::*;

    // Task::new

    #[test]
    fn nueva_tarea_tiene_estado_pendiente() {
        let t = Task::new(1, "Prueba".to_string());
        assert_eq!(t.status, TaskStatus::Pendiente);
    }

    #[test]
    fn nueva_tarea_tiene_descripcion_correcta() {
        let t = Task::new(42, "Mi tarea".to_string());
        assert_eq!(t.description, "Mi tarea");
        assert_eq!(t.id, 42);
    }

    #[test]
    fn created_at_igual_a_updated_at_en_tarea_nueva() {
        let t = Task::new(1, "Test".to_string());
        assert_eq!(t.created_at, t.updated_at);
    }

    //update_description

    #[test]
    fn update_description_cambia_descripcion() {
        let mut t = Task::new(1, "Original".to_string());
        t.update_description("Nuevo texto".to_string());
        assert_eq!(t.description, "Nuevo texto");
    }

    #[test]
    fn update_description_cambia_updated_at() {
        let mut t = Task::new(1, "Original".to_string());
        let before = t.updated_at;
        std::thread::sleep(std::time::Duration::from_millis(10));
        t.update_description("Nuevo".to_string());
        assert!(t.updated_at > before);
    }

    //set_status

    #[test]
    fn set_status_cambia_el_estado() {
        let mut t = Task::new(1, "Test".to_string());
        t.set_status(TaskStatus::Hecho);
        assert_eq!(t.status, TaskStatus::Hecho);
    }

    #[test]
    fn set_status_cambia_updated_at() {
        let mut t = Task::new(1, "Test".to_string());
        let before = t.updated_at;
        std::thread::sleep(std::time::Duration::from_millis(10));
        t.set_status(TaskStatus::EnCurso);
        assert!(t.updated_at > before);
    }

    //TaskStatus::from_str_ci

    #[test]
    fn from_str_ci_acepta_pendiente() {
        assert_eq!(TaskStatus::from_str_ci("pendiente"), Some(TaskStatus::Pendiente));
        assert_eq!(TaskStatus::from_str_ci("PENDIENTE"), Some(TaskStatus::Pendiente));
        assert_eq!(TaskStatus::from_str_ci("Pendiente"), Some(TaskStatus::Pendiente));
    }

    #[test]
    fn from_str_ci_acepta_variantes_en_curso() {
        assert_eq!(TaskStatus::from_str_ci("en curso"),  Some(TaskStatus::EnCurso));
        assert_eq!(TaskStatus::from_str_ci("en_curso"),  Some(TaskStatus::EnCurso));
        assert_eq!(TaskStatus::from_str_ci("encurso"),   Some(TaskStatus::EnCurso));
    }

    #[test]
    fn from_str_ci_acepta_hecho() {
        assert_eq!(TaskStatus::from_str_ci("hecho"), Some(TaskStatus::Hecho));
        assert_eq!(TaskStatus::from_str_ci("HECHO"), Some(TaskStatus::Hecho));
    }

    #[test]
    fn from_str_ci_rechaza_valor_invalido() {
        assert_eq!(TaskStatus::from_str_ci("completado"), None);
        assert_eq!(TaskStatus::from_str_ci(""),           None);
        assert_eq!(TaskStatus::from_str_ci("123"),        None);
    }

    //TaskStatus::Display

    #[test]
    fn display_muestra_texto_legible() {
        assert_eq!(TaskStatus::Pendiente.to_string(), "Pendiente");
        assert_eq!(TaskStatus::EnCurso.to_string(),   "En Curso");
        assert_eq!(TaskStatus::Hecho.to_string(),     "Hecho");
    }
}