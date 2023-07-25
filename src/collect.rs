use std::fs;

use crate::state::DateMetadata;
use walkdir;

fn is_relevant(entry: &walkdir::DirEntry, file_ext: &str) -> bool {
    println!("Checking {}...", entry.file_name().to_str().unwrap());
    entry.file_name()
         .to_str()
         .map(|s| s.ends_with(&file_ext))
         .unwrap_or(false)
}

pub fn get_recent_stats(path: &str) -> DateMetadata {
    let file_ext = ".md";
    let walker = walkdir::WalkDir::new(path).into_iter();
    for entry in walker.filter_entry(|e| is_relevant(e, file_ext)){
        let entry = entry.unwrap();
        let entry_path = entry.path();
        let last_modified = fs::metadata(entry_path).unwrap().modified();
        println!("{} last modified on {:?}", entry_path.display(), last_modified)
    }
    DateMetadata{
        date: chrono::Utc::now().date_naive(),
        updates: 0,
        creations: 0,
    }
}
