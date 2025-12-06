use crate::app::FileExplorerApp;
use crate::services::filesystem;
use crate::ui::context_menu;
use eframe::egui;
use egui_extras::{Column, TableBuilder};
use std::path::PathBuf;

pub fn entries_table(
    app: &mut FileExplorerApp,
    ui: &mut egui::Ui,
    current_time: f64,
    selected_path: &mut Option<PathBuf>,
    folder_to_bookmark: &mut Option<PathBuf>,
) {

    egui::ScrollArea::vertical().show(ui, |ui| {
        TableBuilder::new(ui)
            .striped(true)
            .resizable(true)
            .cell_layout(egui::Layout::left_to_right(egui::Align::Center))
            .column(Column::auto().at_least(300.0))
            .column(Column::auto().at_least(100.0))
            .column(Column::auto().at_least(180.0))
            .column(Column::auto().at_least(100.0))
            .header(25.0, |mut header| {
                header.col(|ui| {
                    ui.strong("Name");
                });
                header.col(|ui| {
                    ui.strong("Type");
                });
                header.col(|ui| {
                    ui.strong("Date Modified");
                });
                header.col(|ui| {
                    ui.strong("Size");
                });
            })
            .body(|mut body| {
                let entries = app.entries.clone();
                for (index, entry) in entries.iter().enumerate() {
                    let row_height = 20.0;
                    body.row(row_height, |mut row| {
                        let is_selected = app.selected_index == Some(index);
                        let mut row_response: Option<egui::Response> = None;

                        row.col(|ui| {
                            let icon = if entry.is_dir { "📁" } else { "📄" };
                            let label = ui
                                .selectable_label(is_selected, format!("{} {}", icon, entry.name));
                            row_response = Some(match row_response.take() {
                                Some(r) => r.union(label),
                                None => label,
                            });

                            row_response.as_ref().unwrap().context_menu(|ui| {
                                context_menu::show_context_menu(
                                    ui,
                                    entry,
                                    selected_path,
                                    folder_to_bookmark,
                                );
                            });
                        });

                        row.col(|ui| {
                            let file_type = if entry.is_dir {
                                "Folder".to_string()
                            } else {
                                entry
                                    .path
                                    .extension()
                                    .and_then(|e| e.to_str())
                                    .map(|e| e.to_uppercase())
                                    .unwrap_or_else(|| "File".to_string())
                            };
                            let r = ui.label(file_type);
                            row_response = Some(match row_response.take() {
                                Some(prev) => prev.union(r),
                                None => r,
                            });
                        });

                        row.col(|ui| {
                            let modified = entry
                                .modified
                                .map(|t| filesystem::format_time(t))
                                .unwrap_or_else(|| "Unknown".to_string());
                            let r = ui.label(modified);
                            row_response = Some(match row_response.take() {
                                Some(prev) => prev.union(r),
                                None => r,
                            });
                        });

                        row.col(|ui| {
                            let size = entry
                                .size
                                .map(|s| filesystem::format_size(s))
                                .unwrap_or_else(|| "".to_string());
                            let r = ui.label(size);
                            row_response = Some(match row_response.take() {
                                Some(prev) => prev.union(r),
                                None => r,
                            });
                        });

                        if let Some(resp) = row_response {
                            if resp.clicked() {
                                app.selected_index = Some(index);
                                let is_double_click = app.last_click_index == Some(index)
                                    && (current_time - app.last_click_time) < 0.3;

                                if is_double_click {
                                    if entry.is_dir {
                                        *selected_path = Some(entry.path.clone());
                                    } else {
                                        #[cfg(target_os = "windows")]
                                        {
                                            let _ = std::process::Command::new("cmd")
                                                .args([
                                                    "/C",
                                                    "start",
                                                    "",
                                                    entry.path.to_str().unwrap_or(""),
                                                ])
                                                .spawn();
                                        }
                                    }
                                    app.last_click_index = None;
                                } else {
                                    app.last_click_time = current_time;
                                    app.last_click_index = Some(index);
                                }
                            }
                            if resp.secondary_clicked() {
                                app.selected_index = Some(index);
                            }
                        }
                    });
                }
            });
    });
}
