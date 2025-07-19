use eframe::egui;
use crate::data::load_array_from_json;
use crate::renderer::CanvasRenderer;

pub struct MyGameApp {
    renderer: Option<CanvasRenderer>,
    json_input: String,
    error_message: Option<String>,
    cell_size: f32,
}

impl Default for MyGameApp {
    fn default() -> Self {
        Self {
            renderer: None,
            json_input: r#"[
  [0, 1, 0, 1, 0],
  [1, 0, 1, 0, 1],
  [0, 1, 0, 1, 0],
  [1, 0, 1, 0, 1],
  [0, 1, 0, 1, 0]
]"#.to_string(),
            error_message: None,
            cell_size: 20.0,
        }
    }
}

impl eframe::App for MyGameApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.heading("2D Array Visualizer");
            ui.separator();

            self.render_input_section(ui);
            self.render_control_section(ui);
            self.render_error_section(ui);
            
            ui.separator();
            
            self.render_canvas_section(ui);
        });
    }
}

impl MyGameApp {
    /// JSON入力エリアの描画
    fn render_input_section(&mut self, ui: &mut egui::Ui) {
        ui.horizontal(|ui| {
            ui.label("JSON Input:");
            if ui.button("Load Sample").clicked() {
                self.load_sample_data();
            }
        });

        ui.add(
            egui::TextEdit::multiline(&mut self.json_input)
                .desired_rows(10)
                .desired_width(f32::INFINITY),
        );
    }

    /// コントロールセクション（ボタンやスライダー）の描画
    fn render_control_section(&mut self, ui: &mut egui::Ui) {
        ui.horizontal(|ui| {
            if ui.button("Load Array").clicked() {
                self.load_array_from_input();
            }

            ui.separator();

            ui.label("Cell Size:");
            if ui.add(egui::Slider::new(&mut self.cell_size, 5.0..=50.0)).changed() {
                if let Some(ref mut renderer) = self.renderer {
                    renderer.set_cell_size(self.cell_size);
                }
            }
        });
    }

    /// エラーメッセージセクションの描画
    fn render_error_section(&mut self, ui: &mut egui::Ui) {
        if let Some(ref error) = self.error_message {
            ui.colored_label(egui::Color32::RED, format!("Error: {}", error));
        }
    }

    /// キャンバスセクションの描画
    fn render_canvas_section(&mut self, ui: &mut egui::Ui) {
        if let Some(ref renderer) = self.renderer {
            ui.label("Canvas (0: Black, 1: White):");
            egui::ScrollArea::both().show(ui, |ui| {
                renderer.render(ui);
            });
        } else {
            ui.label("Load an array to see the visualization");
        }
    }

    fn load_array_from_input(&mut self) {
        match load_array_from_json(&self.json_input) {
            Ok(array) => {
                let mut renderer = CanvasRenderer::new(array);
                renderer.set_cell_size(self.cell_size);
                self.renderer = Some(renderer);
                self.error_message = None;
            }
            Err(e) => {
                self.error_message = Some(e.to_string());
                self.renderer = None;
            }
        }
    }

    fn load_sample_data(&mut self) {
        self.json_input = r#"[
  [1, 1, 1, 1, 1, 0, 0, 0, 0, 0],
  [1, 0, 0, 0, 1, 0, 1, 1, 1, 0],
  [1, 0, 1, 0, 1, 0, 1, 0, 1, 0],
  [1, 0, 0, 0, 1, 0, 1, 1, 1, 0],
  [1, 1, 1, 1, 1, 0, 0, 0, 0, 0],
  [0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
  [0, 1, 1, 1, 0, 0, 1, 1, 1, 0],
  [0, 1, 0, 0, 0, 0, 1, 0, 0, 1],
  [0, 1, 1, 1, 0, 0, 1, 1, 1, 0],
  [0, 0, 0, 1, 0, 0, 0, 0, 0, 1]
]"#.to_string();
    }
}
