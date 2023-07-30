use std::{path, fs::{self, OpenOptions}, io, collections::{BTreeMap}};
use chrono;
use serde;

#[derive(serde::Serialize, serde::Deserialize)]
pub struct DateMetadata {
    // store aggregate counters vs Vec<String> of files that
    // were either modified or created as we can't easily have unique
    // identifiers across file renames for files in the directory
    pub updates: i8,
    pub creations: i8
}

#[derive(serde::Serialize, serde::Deserialize, Default)]
pub struct State {
    pub entries: BTreeMap<chrono::naive::NaiveDate, DateMetadata>
}

pub fn determine_statefile(path: String) -> StateFile {
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

    pub fn touch(&self) -> io::Result<()> {
        match self {
            StateFile::Local(path) => {
                let file = OpenOptions::new().create(true).write(true).open(path).unwrap();
                let default: State = Default::default();
                serde_json::to_writer_pretty(&file, &default)?;
                Ok(())
            },
            // TODO: Remote impl
            StateFile::Remote(path) => {
                OpenOptions::new().create(true).write(true).open(path).unwrap();
                Ok(())
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
            StateFile::Remote(_) => State{entries: BTreeMap::new()}
        }
    }
}
