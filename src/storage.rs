use std::fs;
use std::path::Path;
use crate::task::Task;   

const FILENAME: &str = "tasks.json";   

pub fn load_tasks() -> Result<Vec<Task>, Box<dyn std::error::Error>> {
    if !Path::new(FILENAME).exists() {
        return Ok(vec![]);
    }
    let data = fs::read_to_string(FILENAME)?;
    let tasks: Vec<Task> = serde_json::from_str(&data)?;   
    Ok(tasks)
}

pub fn save_tasks(tasks: &[Task]) -> Result<(), Box<dyn std::error::Error>> {
    let data = serde_json::to_string_pretty(tasks)?;
    fs::write(FILENAME, data)?;
    Ok(())
}