extern crate walkdir;

use walkdir::{WalkDir, DirEntry};

fn main() -> Result<(), std::io::Error> {

    // WalkDir
    for entry in WalkDir::new("/tmp") {
        println!("{}", entry?.path().display());
    }

    // ignore errors (skip directories that no permission to access)
    for entry in WalkDir::new("/tmp").into_iter().filter_map(|e| e.ok()) {
        println!("{}", entry.path().display());
    }

    // follow_links
    for entry in WalkDir::new("/tmp").follow_links(true) {
        println!("{}", entry?.path().display());
    }

    // filter hidden files
    for entry in WalkDir::new("/tmp").into_iter().filter_entry(|e| !is_hidden(e)) {
        println!("{}", entry?.path().display());
    }

    Ok(())
}


fn is_hidden(entry: &DirEntry) -> bool {
    entry.file_name()
         .to_str()
         .map(|s| s.starts_with("."))
         .unwrap_or(false)
}
