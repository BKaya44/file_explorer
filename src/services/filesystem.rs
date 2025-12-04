use crate::model::DirEntry;
use std::fs;
use std::path::PathBuf;
use std::time::{SystemTime, UNIX_EPOCH};

pub fn detect_drives() -> Vec<String> {
    let mut drives = Vec::new();
    for letter in b'A'..=b'Z' {
        let drive = format!("{}:\\", letter as char);
        let path = PathBuf::from(&drive);
        if path.exists() {
            drives.push(drive);
        }
    }
    drives
}

pub fn read_directory(path: &PathBuf) -> Result<Vec<DirEntry>, String> {
    let mut dirs = Vec::new();
    let mut files = Vec::new();

    let entries = fs::read_dir(path).map_err(|e| format!("Error reading directory: {}", e))?;
    for entry in entries.flatten() {
        if let Ok(metadata) = entry.metadata() {
            let name = entry.file_name().to_string_lossy().to_string();
            let size = if metadata.is_file() {
                Some(metadata.len())
            } else {
                None
            };
            let modified = metadata.modified().ok();
            let dir_entry = DirEntry {
                name,
                path: entry.path(),
                is_dir: metadata.is_dir(),
                size,
                modified,
            };
            if dir_entry.is_dir {
                dirs.push(dir_entry);
            } else {
                files.push(dir_entry);
            }
        }
    }

    dirs.sort_by(|a, b| a.name.to_lowercase().cmp(&b.name.to_lowercase()));
    files.sort_by(|a, b| a.name.to_lowercase().cmp(&b.name.to_lowercase()));

    let mut all = dirs;
    all.extend(files);
    Ok(all)
}

pub fn format_size(size: u64) -> String {
    const KB: u64 = 1024;
    const MB: u64 = KB * 1024;
    const GB: u64 = MB * 1024;

    if size >= GB {
        format!("{:.2} GB", size as f64 / GB as f64)
    } else if size >= MB {
        format!("{:.2} MB", size as f64 / MB as f64)
    } else if size >= KB {
        format!("{:.2} KB", size as f64 / KB as f64)
    } else {
        format!("{} B", size)
    }
}

pub fn format_time(time: SystemTime) -> String {
    if let Ok(duration) = time.duration_since(UNIX_EPOCH) {
        let secs = duration.as_secs();
        if let Some(dt) = chrono::DateTime::from_timestamp(secs as i64, 0) {
            return dt.format("%Y-%m-%d %H:%M:%S").to_string();
        }
    }
    "Unknown".to_string()
}
