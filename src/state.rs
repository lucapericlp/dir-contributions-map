use std::{path, fs};
use chrono;
use serde;

#[derive(serde::Serialize, serde::Deserialize)]
struct DateMetadata {
    date: chrono::DateTime<chrono::Utc>,
    updates: i8,
    creations: i8
}

#[derive(serde::Serialize, serde::Deserialize, Default)]
pub struct State {
    entries: Vec<DateMetadata>
}

pub enum StateFile {
    Local(String),
    Remote(String)
}

impl StateFile {
    pub fn exists(&self) -> bool {
        match self {
            StateFile::Local(path) => path::Path::new(path).exists(),
            StateFile::Remote(path) => path::Path::new(path).exists(),
        }
    }

    pub fn build_state(&self) -> State {
        match self {
            StateFile::Local(path) => {
                let file = fs::File::open(path)
                    .expect("file should open read only");
                let state: State = serde_json::from_reader(file)
                    .expect("file should be proper JSON");
                state
            }
            StateFile::Remote(_) => State{entries: Vec::new()}
        }
    }
}
