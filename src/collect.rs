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
                match _entry.metadata() {
                    Ok(metadata) => {
                        if let Ok(created_time) = metadata.created() {
                            // NOTE TO SELF: this mutating business is turning out to be more of a
                            // shit show than I thought, let's return a hashmap local to the
                            // changes and handle resolving with the outer state file elsewhere
                            // TODO: for tomorrow :)
                            let dt: DateTime<Utc> = created_time.into();
                            let dtn = dt.date_naive();
                            let state_date_metadata: Option<&DateMetadata> = state.entries.get(&dtn);
                            let elapsed_since = chrono::Duration::from_std(created_time.elapsed().unwrap());
                            if elapsed_since.unwrap() <= recency {
                                state.entries.entry(dtn).and_modify(|k| k.updates += 1);
                                match state_date_metadata {
                                    Some(value) => {
                                        state.entries.entry(dtn).and_modify(|k| k.updates += 1);
                                        // state.entries[&dtn] = DateMetadata{
                                        //     updates: value.updates, creations: value.creations+1
                                        // }
                                    }
                                    None => {
                                        state.entries
                                        state.entries[&dtn] = DateMetadata{updates: 0, creations: 1};
                                    }
                                }
                                // skip modified check if its been created in recency window
                                // otherwise, still perform modified check
                                continue
                            }
                        }
                        if let Ok(modified_time) = metadata.modified() {
                            let dt: DateTime<Utc> = modified_time.into();
                            let dtn = dt.date_naive();
                            let state_date_metadata: Option<&DateMetadata> = state.entries.get(&dtn);
                            let elapsed_since = chrono::Duration::from_std(modified_time.elapsed().unwrap());
                            if elapsed_since.unwrap() <= recency {
                                match state_date_metadata {
                                    Some(value) => {
                                        state.entries[&dtn] = DateMetadata{
                                            updates: value.updates+1, creations: value.creations
                                        }
                                    }
                                    None => {
                                        panic!("Attempting to increment update counter for date {:?} that was never in the state file! This is an indication that the state file is corrupt.", dtn);
                                        // state.entries[&dtn] = DateMetadata{updates: 1, creations: 0};
                                    }
                                }
                            }
                        }
                    }
                    Err(e) => {
                        eprintln!("Failed to get metadata for file {}: {}", _entry.path().display(), e);
                    }
                }
            }
        }
    }
    Ok(())
}
