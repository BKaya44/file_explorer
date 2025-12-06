use std::path::PathBuf;

pub fn show_context_menu(
    ui: &mut egui::Ui,
    entry: &crate::model::DirEntry,
    selected_path: &mut Option<PathBuf>,
    folder_to_bookmark: &mut Option<PathBuf>,
) {
    let path = &entry.path;
    if ui.button("Open").clicked() {
        if path.is_dir() {
            *selected_path = Some(path.clone());
        } else if path.is_file() {
            if let Err(err) = open::that(path) {
                eprintln!("Failed to open file externally: {err}");
            }
        }

        ui.close();
    }

    if ui.button("Copy").clicked() {
        // TODO: Implement copy
        ui.close();
    }

    if ui.button("Delete").clicked() {
        // TODO: Implement delete
        ui.close();
    }

    ui.separator();

    if ui.button("Rename").clicked() {
        // TODO: Implement rename
        ui.close();
    }

    if path.is_dir() {
        if ui.button("Add to bookmarks").clicked() {
            *folder_to_bookmark = Some(entry.path.clone());
            ui.close();
        }
    }

    if ui.button("Properties").clicked() {
        // TODO: Implement properties dialog
        ui.close();
    }
}
