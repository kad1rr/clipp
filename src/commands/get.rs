use std::fs::File;
use std::io::Read;
use dirs;
use serde::{Deserialize, Serialize};
use crate::error;

#[derive(Debug, Serialize, Deserialize)]
struct ClipData {
    filename: String,
    data: String,
}

pub struct Get {}

impl Get {
    pub fn get(filename: String) -> Result<Option<String>, String> {
        let mut path = dirs::home_dir().ok_or("Error: Could not determine home directory")?;
        path.push("clip_data.json");

        let file = File::open(&path);

        match file {
            Ok(f) => {
                let mut content = String::new();
                f.take(u64::MAX as u64) // Read at most u64::MAX bytes
                    .read_to_string(&mut content)
                    .map_err(|e| e.to_string())?;

                let clip_data: Vec<ClipData> = serde_json::from_str(&content).unwrap_or_default();

                if let Some(existing_data) = clip_data.iter().find(|d| d.filename == filename) {
                    Ok(Some(existing_data.data.clone()))
                } else {
                    Ok(None)
                }
            }
            Err(_) => Err(error::no_file::no_file_founded()),
        }
    }
}