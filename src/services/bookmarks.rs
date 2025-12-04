use std::path::PathBuf;

pub fn add_bookmark(bookmarks: &mut Vec<PathBuf>, path: PathBuf) {
    if !bookmarks.contains(&path) {
        bookmarks.push(path);
    }
}

pub fn remove_bookmark(bookmarks: &mut Vec<PathBuf>, index: usize) {
    if index < bookmarks.len() {
        bookmarks.remove(index);
    }
}
