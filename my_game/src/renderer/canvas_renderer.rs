use eframe::egui;
use ndarray::Array2;

pub struct CanvasRenderer {
    array: Array2<i32>,
    cell_size: f32,
}

impl CanvasRenderer {
    pub fn new(array: Array2<i32>) -> Self {
        Self {
            array,
            cell_size: 20.0, // デフォルトのセルサイズ
        }
    }

    pub fn set_cell_size(&mut self, size: f32) {
        self.cell_size = size;
    }

    pub fn get_canvas_size(&self) -> egui::Vec2 {
        let (rows, cols) = self.array.dim();
        egui::Vec2::new(
            cols as f32 * self.cell_size,
            rows as f32 * self.cell_size,
        )
    }

    pub fn render(&self, ui: &mut egui::Ui) {
        let canvas_size = self.get_canvas_size();
        let (rows, cols) = self.array.dim();

        let (response, painter) = ui.allocate_painter(canvas_size, egui::Sense::hover());

        // 背景を黒で塗りつぶし
        painter.rect_filled(
            response.rect,
            egui::CornerRadius::ZERO,
            egui::Color32::BLACK,
        );

        // 各セルを描画
        for row in 0..rows {
            for col in 0..cols {
                let value = self.array[[row, col]];
                
                if value == 1 {
                    // 1の場合は白で描画
                    let cell_rect = egui::Rect::from_min_size(
                        response.rect.min + egui::Vec2::new(
                            col as f32 * self.cell_size,
                            row as f32 * self.cell_size,
                        ),
                        egui::Vec2::splat(self.cell_size),
                    );

                    painter.rect_filled(
                        cell_rect,
                        egui::CornerRadius::ZERO,
                        egui::Color32::WHITE,
                    );
                }
            }
        }

        // グリッド線を描画（オプション）
        self.draw_grid(&painter, &response.rect, rows, cols);
    }

    fn draw_grid(&self, painter: &egui::Painter, rect: &egui::Rect, rows: usize, cols: usize) {
        let grid_color = egui::Color32::from_gray(64);
        let stroke = egui::Stroke::new(1.0, grid_color);

        // 垂直線
        for col in 0..=cols {
            let x = rect.min.x + col as f32 * self.cell_size;
            painter.line_segment(
                [egui::pos2(x, rect.min.y), egui::pos2(x, rect.max.y)],
                stroke,
            );
        }

        // 水平線
        for row in 0..=rows {
            let y = rect.min.y + row as f32 * self.cell_size;
            painter.line_segment(
                [egui::pos2(rect.min.x, y), egui::pos2(rect.max.x, y)],
                stroke,
            );
        }
    }
}
