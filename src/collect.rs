use std::collections::HashMap;

use crate::state::DateMetadata;
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


pub fn get_recent_stats(
    path: &str, recency: chrono::Duration
) -> Result<HashMap<chrono::naive::NaiveDate, DateMetadata>, ()>{
    let file_ext = ".md";
    let walker = walkdir::WalkDir::new(path).into_iter();
    let mut discoveries = HashMap::new();
    for entry in walker.filter_entry(|e| is_relevant(e, file_ext, recency)){
        let _entry = entry.as_ref().unwrap();
        match _entry.file_type().is_dir() {
            true => continue,
            false => {
                match _entry.metadata() {
                    Ok(metadata) => {
                        // note: we shouldn't need to check again whether it was created or
                        // modified in the recency window, alas we move
                        if let Ok(created_time) = metadata.created() {
                            let dt: chrono::DateTime<chrono::Utc> = created_time.into();
                            let dtn = dt.date_naive();
                            let elapsed_since = chrono::Duration::from_std(created_time.elapsed().unwrap());
                            if elapsed_since.unwrap() <= recency {
                                discoveries.entry(dtn).or_insert(DateMetadata::default()).creations += 1;
                            }
                            // skip modified check if its been created in recency window
                            // otherwise, still perform modified check
                            continue
                        }
                        if let Ok(modified_time) = metadata.modified() {
                            let dt: chrono::DateTime<chrono::Utc> = modified_time.into();
                            let dtn = dt.date_naive();
                            let elapsed_since = chrono::Duration::from_std(modified_time.elapsed().unwrap());
                            if elapsed_since.unwrap() <= recency {
                                discoveries.entry(dtn).or_insert(DateMetadata::default()).creations += 1;
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
    Ok(discoveries)
}
