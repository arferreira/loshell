use std::fs;
use std::path::PathBuf;

use serde::{Deserialize, Serialize};

use crate::todo::Task;

#[derive(Serialize, Deserialize, Default)]
pub struct TaskData {
    pub tasks: Vec<Task>,
    pub next_id: u64,
}

pub fn get_data_path() -> PathBuf {
    let base = dirs::data_dir().unwrap_or_else(|| PathBuf::from("."));
    base.join("loshell").join("tasks.json")
}

pub fn load_tasks() -> TaskData {
    let path = get_data_path();
    match fs::read_to_string(&path) {
        Ok(content) => serde_json::from_str(&content).unwrap_or_default(),
        Err(_) => TaskData::default(),
    }
}

pub fn save_tasks(data: &TaskData) {
    let path = get_data_path();
    if let Some(parent) = path.parent() {
        let _ = fs::create_dir_all(parent);
    }
    if let Ok(json) = serde_json::to_string_pretty(data) {
        let _ = fs::write(&path, json);
    }
}
