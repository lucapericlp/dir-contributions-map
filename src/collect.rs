use std::collections::HashMap;

use crate::state::DateMetadata;
use walkdir;

fn is_relevant(
    entry: &walkdir::DirEntry, file_ext: &str, recency: chrono::Duration
) -> bool {
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
    let mut discoveries: HashMap<chrono::NaiveDate, DateMetadata> = HashMap::new();
    for entry in walker.filter_entry(|e| is_relevant(e, file_ext, recency)){
        let _entry = entry.as_ref().unwrap();
        match _entry.file_type().is_dir() {
            true => continue,
            false => {
                match _entry.metadata() {
                    Ok(metadata) => {
                        // we only check if it was modified since rclone copy will keep the
                        // modified date in metadata but not the created date
                        if let Ok(modified_time) = metadata.modified() {
                            let dt: chrono::DateTime<chrono::Utc> = modified_time.into();
                            let dtn = dt.date_naive();
                            let elapsed_since = chrono::Duration::from_std(modified_time.elapsed().unwrap());
                            if elapsed_since.unwrap() <= recency {
                                println!("Adding {:?}", _entry.path());
                                discoveries.entry(dtn)
                                    .and_modify(|e| e.updates += 1)
                                    .or_insert(DateMetadata::default());
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
