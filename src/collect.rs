use std::{fs, time::SystemTime};

use crate::state::{State, DateMetadata};
use chrono::{NaiveDate, DateTime, Utc};
use walkdir;

fn is_relevant(
    entry: &walkdir::DirEntry, file_ext: &str, recency: chrono::Duration
) -> bool {
    println!("Checking {}...", entry.file_name().to_str().unwrap());
    if entry.file_type().is_dir(){
        return true;
    }

    let is_markdown_file = entry.file_name()
        .to_str()
        .map(|s| s.ends_with(&file_ext))
        .unwrap_or(false);

    if !is_markdown_file {
        return false;
    }

    match entry.metadata() {
        Ok(metadata) => {
            if let Ok(created_time) = metadata.created() {
                let elapsed_since = chrono::Duration::from_std(created_time.elapsed().unwrap());
                if elapsed_since.unwrap() <= recency {
                    return true;
                    // still check if it was modified in this window
                }
            }
            if let Ok(modified_time) = metadata.modified() {
                let elapsed_since = chrono::Duration::from_std(modified_time.elapsed().unwrap());
                return elapsed_since.unwrap() <= recency;
            }
        }
        Err(e) => {
            eprintln!("Failed to get metadata for file {}: {}", entry.path().display(), e);
            return false;
        }
    }

    false
}

pub fn add_recent_stats(
    path: &str, state: &mut State, recency: chrono::Duration
) -> Result<(), ()>{
    let file_ext = ".md";
    let walker = walkdir::WalkDir::new(path).into_iter();
    for entry in walker.filter_entry(|e| is_relevant(e, file_ext, recency)){
        let _entry = entry.as_ref().unwrap();
        match _entry.file_type().is_dir() {
            true => continue,
            false => {
                let entry_path = _entry.path();
                let md = fs::metadata(entry_path).unwrap();
                let dt_created: DateTime<Utc> = md.created().unwrap().into();
                let dtn_created = dt_created.date_naive();
                let last_modified = md.modified();
                println!("{} last modified on {:?}", entry_path.display(), last_modified)
                // if state.entries.contains_key(&dtn_created){
                //      // test
                //     state.entries.get(&dtn_created);
                // } else {
                //     // new
                // }
            }
        }
    }
    Ok(())
}
