use crate::app::FileExplorerApp;
use eframe::egui;
use std::path::PathBuf;

pub fn top_bar(app: &mut FileExplorerApp, ctx: &egui::Context) {
    egui::TopBottomPanel::top("top_panel").show(ctx, |ui| {
        ui.horizontal(|ui| {
            let path_str = app.current_path.to_string_lossy().to_string();
            let parts: Vec<&str> = if cfg!(target_os = "windows") {
                path_str.split('\\').filter(|s| !s.is_empty()).collect()
            } else {
                path_str.split('/').filter(|s| !s.is_empty()).collect()
            };

            for (i, part) in parts.iter().enumerate() {
                if ui.button(part.to_string()).clicked() {
                    let mut new_path = PathBuf::new();
                    for j in 0..=i {
                        new_path.push(parts[j]);
                    }
                    app.navigate_to(new_path);
                }
                if i < parts.len() - 1 {
                    ui.label(">");
                }
            }
        });
    });
}

pub fn side_panel(app: &mut FileExplorerApp, ctx: &egui::Context) {
    egui::SidePanel::left("left_panel")
        .min_width(200.0)
        .show(ctx, |ui| {
            ui.heading("Drives");
            ui.separator();

            let mut selected_drive: Option<String> = None;
            for drive in &app.drives {
                if ui.button(format!("💾 {}", drive)).clicked() {
                    selected_drive = Some(drive.clone());
                }
            }

            if let Some(drive) = selected_drive {
                app.navigate_to(PathBuf::from(drive));
            }

            ui.add_space(10.0);
            ui.heading("Bookmarks");
            ui.separator();

            let mut bookmark_to_navigate: Option<PathBuf> = None;
            let mut bookmark_to_remove: Option<usize> = None;

            for (i, bookmark) in app.bookmarks.iter().enumerate() {
                let bookmark_name = bookmark
                    .file_name()
                    .and_then(|n| n.to_str())
                    .unwrap_or(bookmark.to_str().unwrap_or("Unknown"));
                let response = ui.button(format!("⭐ {}", bookmark_name));

                if response.clicked() {
                    bookmark_to_navigate = Some(bookmark.clone());
                }

                response.context_menu(|ui| {
                    if ui.button("Remove from bookmarks").clicked() {
                        bookmark_to_remove = Some(i);
                        ui.close();
                    }
                });
            }

            if let Some(path) = bookmark_to_navigate {
                app.navigate_to(path);
            }
            if let Some(index) = bookmark_to_remove {
                app.remove_bookmark(index);
            }
        });
}

pub fn central_panel(app: &mut FileExplorerApp, ctx: &egui::Context) {
    egui::CentralPanel::default().show(ctx, |ui| {
        if let Some(error) = &app.error_msg {
            ui.colored_label(egui::Color32::RED, error);
            ui.separator();
        }

        let mut selected_path: Option<std::path::PathBuf> = None;
        let mut folder_to_bookmark: Option<std::path::PathBuf> = None;
        let current_time = ui.input(|i| i.time);

        crate::ui::table::entries_table(
            app,
            ui,
            current_time,
            &mut selected_path,
            &mut folder_to_bookmark,
        );

        if let Some(path) = selected_path {
            app.navigate_to(path);
        }
        if let Some(path) = folder_to_bookmark {
            app.add_bookmark(path);
        }
    });
}
