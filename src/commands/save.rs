use std::io::Seek;
use std::fs::{OpenOptions};
use std::io::{Read, Write};
use dirs;
use serde::{Deserialize, Serialize};
use crate::error;

pub struct Save {}

#[derive(Debug, Serialize, Deserialize)]
struct ClipData {
    filename: String,
    data: String,
}

impl Save {
    pub fn save(filename: String, data: &str) -> Result<String, String> {
        let mut path = dirs::home_dir().ok_or("Error: Could not determine home directory")?;
        path.push("clip_data.json");

        let _file_exists = path.exists();

        let mut file = OpenOptions::new()
            .create(true)
            .read(true)
            .write(true)
            .open(&path);

        match &mut file {
            Ok(ref mut f) => {
                let mut content = String::new();
                f.read_to_string(&mut content).map_err(|e| e.to_string())?;

                let mut clip_data: Vec<ClipData> = serde_json::from_str(&content).unwrap_or_default();

                let new_clip_data = ClipData {
                    filename: filename.clone(),
                    data: data.to_string(),
                };

                // Check if the filename already exists in the clip_data vector
                if let Some(existing_data) = clip_data.iter_mut().find(|d| d.filename == filename) {
                    existing_data.data = data.to_string();
                } else {
                    // If filename does not exist, add new data
                    clip_data.push(new_clip_data);
                }

                // Serialize the updated clip_data and write it to the file
                let serialized_clip_data = serde_json::to_string_pretty(&clip_data).map_err(|e| e.to_string())?;
                f.seek(std::io::SeekFrom::Start(0)).map_err(|e| e.to_string())?;
                f.set_len(0).map_err(|e| e.to_string())?;
                f.write_all(serialized_clip_data.as_bytes()).map_err(|e| e.to_string())?;

                Ok(format!("{:?}", path))
            }
            Err(..) => return Err(error::no_file::no_file_founded()),
        }
    }
}