pub mod chat;
pub mod settings;
pub mod sidebar;

use eframe::egui::Ui;

/// Runs `add` in a centered column of at most `width` pixels.
pub fn column<R>(ui: &mut Ui, width: f32, add: impl FnOnce(&mut Ui) -> R) -> R {
    let avail = ui.available_width();
    let w = width.min(avail);
    let pad = ((avail - w) / 2.0).max(0.0);
    ui.horizontal(|ui| {
        ui.add_space(pad);
        ui.vertical(|ui| {
            ui.set_width(w);
            add(ui)
        })
        .inner
    })
    .inner
}
