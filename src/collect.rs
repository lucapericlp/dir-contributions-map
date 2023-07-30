use std::{fs, time::SystemTime};

use crate::state::{State, DateMetadata};
use chrono::{NaiveDate, DateTime, Utc};
use walkdir;

fn is_relevant(entry: &walkdir::DirEntry, file_ext: &str) -> bool {
    println!("Checking {}...", entry.file_name().to_str().unwrap());
    if entry.file_type().is_dir(){
        return true;
    } else {
        return entry.file_name()
            .to_str()
            .map(|s| s.ends_with(&file_ext))
            .unwrap_or(false)
    }
}

pub fn add_recent_stats(path: &str, state: &State) -> Result<(), ()>{
    let file_ext = ".md";
    let walker = walkdir::WalkDir::new(path).into_iter();
    for entry in walker.filter_entry(|e| is_relevant(e, file_ext)){
        let _entry = entry.as_ref().unwrap();
        match _entry.file_type().is_dir() {
            true => continue,
            false => {
                let entry_path = _entry.path();
                let md = fs::metadata(entry_path).unwrap();
                let dt_created: DateTime<Utc> = md.created().unwrap().into();
                let dtn_created = dt_created.date_naive();
                if state.entries.contains_key(&dtn_created){
                     // test
                }
                md.modified();
                state.entries.insert(key, value);
                println!("{} last modified on {:?}", entry_path.display(), last_modified)
            }
        }
    }
    DateMetadata{
        updates: 0,
        creations: 0,
    }
}
