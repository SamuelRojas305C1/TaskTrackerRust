use std::fs;
use std::path::PathBuf;
use std::env;
use crate::task::Task;

/// Devuelve la ruta absoluta a tasks.json:
///   Windows: %USERPROFILE%\.task_tracker\tasks.json
///   Unix:    $HOME/.task_tracker/tasks.json
/// Si no se puede resolver el home, usa el directorio actual.
fn get_tasks_path() -> PathBuf {
    let home = env::var("USERPROFILE")
        .or_else(|_| env::var("HOME"))
        .map(PathBuf::from)
        .unwrap_or_else(|_| PathBuf::from("."));

    home.join(".task_tracker").join("tasks.json")
}

pub fn load_tasks() -> Result<Vec<Task>, String> {
    let path = get_tasks_path();

    if !path.exists() {
        return Ok(vec![]);
    }

    let data = fs::read_to_string(&path)
        .map_err(|e| format!("No se pudo leer '{}': {}", path.display(), e))?;

    let tasks: Vec<Task> = serde_json::from_str(&data)
        .map_err(|e| format!("JSON inválido en '{}': {}", path.display(), e))?;

    Ok(tasks)
}

pub fn save_tasks(tasks: &[Task]) -> Result<(), String> {
    let path = get_tasks_path();

    // Crear el directorio padre si no existe
    if let Some(parent) = path.parent() {
        fs::create_dir_all(parent)
            .map_err(|e| format!("No se pudo crear el directorio '{}': {}", parent.display(), e))?;
    }

    let data = serde_json::to_string_pretty(tasks)
        .map_err(|e| format!("Error al serializar las tareas: {}", e))?;

    fs::write(&path, data)
        .map_err(|e| format!("No se pudo escribir '{}': {}", path.display(), e))?;

    Ok(())
}