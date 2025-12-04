use serde::{Deserialize, Serialize};
use std::fs;
use std::io::{BufReader, Read};
use std::path::PathBuf;

#[derive(Serialize, Deserialize)]
struct BookmarkStore {
    bookmarks: Vec<String>,
}

fn bookmarks_file() -> PathBuf {
    let base_dir = dirs::data_local_dir().unwrap_or_else(|| PathBuf::from("."));
    let mut dir = base_dir;
    dir.push("NewFileExplorer");

    if let Err(e) = fs::create_dir_all(&dir) {
        eprintln!("Failed to create bookmarks directory {:?}: {}", dir, e);
    }

    dir.push("bookmarks.json");
    dir
}

pub fn load_bookmarks() -> Vec<PathBuf> {
    let file_path = bookmarks_file();
    let mut result = Vec::new();

    if let Ok(file) = fs::File::open(&file_path) {
        let mut reader = BufReader::new(file);
        let mut contents = String::new();
        if reader.read_to_string(&mut contents).is_ok() {
            if let Ok(store) = serde_json::from_str::<BookmarkStore>(&contents) {
                for s in store.bookmarks {
                    if !s.trim().is_empty() {
                        let path = PathBuf::from(&s);
                        if path.exists() {
                            result.push(path);
                        }
                    }
                }
            }
        }
    }
    save_bookmarks(&result);

    result
}

fn save_bookmarks(bookmarks: &Vec<PathBuf>) {
    let file_path = bookmarks_file();
    let store = BookmarkStore {
        bookmarks: bookmarks
            .iter()
            .filter_map(|p| p.to_str().map(|s| s.to_string()))
            .collect(),
    };

    if let Ok(json) = serde_json::to_string_pretty(&store) {
        if let Some(parent) = file_path.parent() {
            let _ = fs::create_dir_all(parent);
        }
        if let Ok(mut file) = fs::File::create(file_path) {
            use std::io::Write;
            let _ = file.write_all(json.as_bytes());
        }
    }
}

pub fn add_bookmark(bookmarks: &mut Vec<PathBuf>, path: PathBuf) {
    if !bookmarks.contains(&path) {
        bookmarks.push(path);
        save_bookmarks(bookmarks);
    }
}

pub fn remove_bookmark(bookmarks: &mut Vec<PathBuf>, index: usize) {
    if index < bookmarks.len() {
        bookmarks.remove(index);
        save_bookmarks(bookmarks);
    }
}
