use eframe::egui;
use ndarray::Array2;

mod buttons_renderer;
mod canvas_renderer;

struct CanvasState {
    rows: usize,
    cols: usize,
    array: Array2<i32>,
}

struct MyEguiApp {
    canvas_state: CanvasState,
}

impl MyEguiApp {
    fn new(_cc: &eframe::CreationContext<'_>) -> Self {
        let rows = 5;
        let cols = 5;
        let array = Self::random_array(rows, cols);
        let canvas_state = CanvasState { rows, cols, array };
        Self { canvas_state }
    }

    fn random_array(rows: usize, cols: usize) -> Array2<i32> {
        use rand::Rng;
        let mut rng = rand::rng();
        let data = (0..rows * cols)
            .map(|_| if rng.random_bool(0.5) { 1 } else { 0 })
            .collect();
        Array2::from_shape_vec((rows, cols), data).unwrap()
    }
}

impl eframe::App for MyEguiApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.heading("My App!");

            buttons_renderer::ButtonRenderer::render(
                ui,
                &mut self.canvas_state.rows,
                &mut self.canvas_state.cols,
            );
            // もしrowsやcolsが変更された場合、arrayを再生成
            if self.canvas_state.array.dim()
                != (self.canvas_state.rows, self.canvas_state.cols)
            {
                self.canvas_state.array = Self::random_array(
                    self.canvas_state.rows,
                    self.canvas_state.cols,
                );
            }
            // キャンバス表示
            let canvas = canvas_renderer::CanvasRenderer::new(
                self.canvas_state.array.clone(),
            );
            canvas.render(ui);
        });
    }
}

fn main() {
    let native_options = eframe::NativeOptions::default();
    let _ = eframe::run_native(
        "My egui App",
        native_options,
        Box::new(|cc| Ok(Box::new(MyEguiApp::new(cc)))),
    );
}
