use eframe::egui;

pub struct ButtonRenderer;

impl ButtonRenderer {
    pub fn render(ui: &mut egui::Ui, rows: &mut usize, cols: &mut usize) {
        // 行数・列数の増減ボタン
        ui.horizontal(|ui| {
            if ui.button("- row").clicked() && *rows > 1 {
                *rows -= 1;
            }
            ui.label(format!("rows: {}", rows));
            if ui.button("+ row").clicked() {
                *rows += 1;
            }

            ui.separator();

            if ui.button("- col").clicked() && *cols > 1 {
                *cols -= 1;
            }

            ui.label(format!("cols: {}", cols));
            if ui.button("+ col").clicked() {
                *cols += 1;
            }
        });
    }
}
