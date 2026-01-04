use chrono::prelude::*;
use rand::Rng;

use serde::{Deserialize, Serialize};
use serde_json;
use std::fs;

#[derive(Debug, Serialize, Deserialize,Clone)]
pub struct Note {
    pub id: u32,
    pub content: String,
    pub date: String,
    pub time: String,
    pub tag: String,
}
impl Note {
    pub fn new(content: String, tag: String) -> Self {
        let date = Local::now().format("%Y-%m-%d");
        let time = Local::now().format("%H:%M");
        let id = rand::thread_rng().gen_range(1000..9999);
        Note {
            id: id,
            content: content,
            date: date.to_string(),
            time: time.to_string(),
            tag: tag,
        }
    }
    //load from json file in ~/config/scribo.json
    pub fn load_from_json() -> Option<Vec<Note>> {
        let config_dir = dirs::config_dir().unwrap();
        let file_path = config_dir.join("scribo.json");
        let data = fs::read_to_string(file_path).ok()?;
        let notes: Vec<Note> = serde_json::from_str(&data).ok()?;
        Some(notes)
    }
    //save to json file in ~/config/scribo.json
    pub fn save_to_json(notes: &Vec<Self>) -> Option<bool>   {
        let config_dir = dirs::config_dir().unwrap();
        let file_path = config_dir.join("scribo.json");
        let json = match serde_json::to_string_pretty(notes) {
            Ok(j) => j,
            Err(_) => return Some(false),
        };

        Some(fs::write(file_path, json).is_ok())
    }
}
