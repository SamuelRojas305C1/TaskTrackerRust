use std::fs;
use std::path::Path;
use crate::task::Task;


fn tasks_filename(username: &str) -> String {
    format!("data/{}.json", username)
}


pub fn load_tasks_for_user(username: &str) -> Result<Vec<Task>, Box<dyn std::error::Error>> {
    let filename = tasks_filename(username);
    if !Path::new(&filename).exists() {
        return Ok(vec![]);
    }
    let data = fs::read_to_string(&filename)?;
    let tasks: Vec<Task> = serde_json::from_str(&data)?;
    Ok(tasks)
}


pub fn save_tasks_for_user(username: &str, tasks: &[Task]) -> Result<(), Box<dyn std::error::Error>> {
    let filename = tasks_filename(username);
    
    if let Some(parent) = Path::new(&filename).parent() {
        fs::create_dir_all(parent)?;
    }
    let data = serde_json::to_string_pretty(tasks)?;
    fs::write(filename, data)?;
    Ok(())
}