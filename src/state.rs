use std::{path, fs::{self, OpenOptions}};
use chrono;
use serde;

#[derive(serde::Serialize, serde::Deserialize)]
pub struct DateMetadata {
    pub date: chrono::naive::NaiveDate,
    // store aggregate counters vs Vec<String> of files that
    // were either modified or created as we can't easily have unique
    // identifiers across file renames for files in the directory
    pub updates: i8,
    pub creations: i8
}

#[derive(serde::Serialize, serde::Deserialize, Default)]
pub struct State {
    entries: Vec<DateMetadata>
}

pub fn build_statefile(path: String) -> StateFile {
    // assumption: rclone paths will have to be defined with : while
    // local paths will not
    match path.contains(":") {
      true  => StateFile::Remote(path),
      false => StateFile::Local(path)
    }
}

pub enum StateFile {
    Local(String),
    Remote(String)
}

impl StateFile {
    pub fn exists(&self) -> bool {
        match self {
            StateFile::Local(path) => path::Path::new(path).exists(),
            // TODO: Remote impl
            StateFile::Remote(path) => path::Path::new(path).exists(),
        }
    }

    pub fn touch(&self) -> () {
        match self {
            StateFile::Local(path) => {
                OpenOptions::new().create(true).write(true).open(path).unwrap();
            },
            // TODO: Remote impl
            StateFile::Remote(path) => {
                OpenOptions::new().create(true).write(true).open(path).unwrap();
            }
        }
    }

    pub fn load_state(&self) -> State {
        match self {
            StateFile::Local(path) => {
                let file = fs::File::open(path)
                    .expect("file should open read only");
                let state: State = serde_json::from_reader(file)
                    .expect("file should be proper JSON");
                state
            }
            // TODO: Remote impl
            StateFile::Remote(_) => State{entries: Vec::new()}
        }
    }
}
