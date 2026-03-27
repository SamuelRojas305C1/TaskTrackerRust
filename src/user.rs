use serde::{Serialize, Deserialize};
use std::collections::HashMap;
use std::fs;
use std::path::Path;

const USERS_FILE: &str = "users.json";

#[derive(Serialize, Deserialize, Clone)]
pub struct User {
    pub username: String,
    pub password_hash: String,
}

pub type UserMap = HashMap<String, String>; 


pub fn load_users() -> Result<UserMap, Box<dyn std::error::Error>> {
    if !Path::new(USERS_FILE).exists() {
        return Ok(HashMap::new());
    }
    let data = fs::read_to_string(USERS_FILE)?;
    let users: UserMap = serde_json::from_str(&data)?;
    Ok(users)
}


pub fn save_users(users: &UserMap) -> Result<(), Box<dyn std::error::Error>> {
    let data = serde_json::to_string_pretty(users)?;
    fs::write(USERS_FILE, data)?;
    Ok(())
}


pub fn authenticate(username: &str, password: &str, users: &UserMap) -> bool {
    match users.get(username) {
        Some(hash) => bcrypt::verify(password, hash).unwrap_or(false),
        None => false,
    }
}


pub fn register(username: &str, password: &str, users: &mut UserMap) -> Result<(), String> {
    if users.contains_key(username) {
        return Err("El usuario ya existe".to_string());
    }
    let hash = bcrypt::hash(password, bcrypt::DEFAULT_COST).map_err(|e| e.to_string())?;
    users.insert(username.to_string(), hash);
    Ok(())
}