use crate::model::DirEntry;
use crate::services::{bookmarks, filesystem};
use crate::ui;
use eframe::egui;
use std::path::PathBuf;

pub struct FileExplorerApp {
    pub current_path: PathBuf,
    pub entries: Vec<DirEntry>,
    pub error_msg: Option<String>,
    pub bookmarks: Vec<PathBuf>,
    pub drives: Vec<String>,
    pub selected_index: Option<usize>,
    pub last_click_time: f64,
    pub last_click_index: Option<usize>,
    pub history: Vec<PathBuf>,
    pub history_index: usize,
}

impl Default for FileExplorerApp {
    fn default() -> Self {
        let current_path = std::env::current_dir().unwrap_or_else(|_| PathBuf::from("C:\\"));
        let mut app = Self {
            current_path: current_path.clone(),
            entries: Vec::new(),
            error_msg: None,
            bookmarks: crate::services::bookmarks::load_bookmarks(),
            drives: Vec::new(),
            selected_index: None,
            last_click_time: 0.0,
            last_click_index: None,
            history: vec![current_path.clone()],
            history_index: 0,
        };
        app.drives = filesystem::detect_drives();
        app.reload_directory();
        app
    }
}

impl FileExplorerApp {
    pub fn reload_directory(&mut self) {
        self.entries.clear();
        self.error_msg = None;
        self.selected_index = None;
        match filesystem::read_directory(&self.current_path) {
            Ok(entries) => self.entries = entries,
            Err(e) => self.error_msg = Some(e),
        }
    }

    pub fn navigate_to(&mut self, path: PathBuf) {
        self.history.truncate(self.history_index + 1);
        self.history.push(path.clone());
        self.history_index = self.history.len() - 1;
        self.current_path = path;
        self.reload_directory();
    }

    pub fn add_bookmark(&mut self, path: PathBuf) {
        bookmarks::add_bookmark(&mut self.bookmarks, path);
    }

    pub fn remove_bookmark(&mut self, index: usize) {
        bookmarks::remove_bookmark(&mut self.bookmarks, index);
    }

    pub fn go_back(&mut self) {
        if self.history_index > 0 {
            self.history_index -= 1;
            self.current_path = self.history[self.history_index].clone();
            self.reload_directory();
        }
    }

    pub fn go_forward(&mut self) {
        if self.history_index < self.history.len() - 1 {
            self.history_index += 1;
            self.current_path = self.history[self.history_index].clone();
            self.reload_directory();
        }
    }
}

impl eframe::App for FileExplorerApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        ui::draw(self, ctx);
    }
}
