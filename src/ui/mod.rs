mod context_menu;
mod layout;
mod table;

use crate::app::FileExplorerApp;
use eframe::egui;

pub fn draw(app: &mut FileExplorerApp, ctx: &egui::Context) {
    layout::top_bar(app, ctx);
    layout::side_panel(app, ctx);
    layout::central_panel(app, ctx);
}
