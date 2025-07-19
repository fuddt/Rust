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
        let (rows, cols) : (usize, usize) = self.array.dim();
        egui::Vec2::new(
            cols as f32 * self.cell_size,
            rows as f32 * self.cell_size,
        )
    }

    pub fn render(&self, ui: &mut egui::Ui) {
        let canvas_size = self.get_canvas_size();
        let (rows, cols) = self.array.dim();

        // ResponseとPainterを取得　ResponseはUIの反応を検知するメソッドとか入ってる、Painterは描画を担当
        let (response, painter): (egui::Response, egui::Painter) = ui.allocate_painter(canvas_size, egui::Sense::hover());
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

#[cfg(test)]
mod tests {
    use super::*;
    use ndarray::Array2;

    #[test]
    fn test_set_cell_size() {
        let array = Array2::from_shape_vec((2, 2), vec![0, 1, 1, 0]).unwrap();
        let mut renderer = CanvasRenderer::new(array);
        
        // デフォルトのセルサイズは20.0
        assert_eq!(renderer.cell_size, 20.0);
        
        // セルサイズを変更
        renderer.set_cell_size(30.0);
        assert_eq!(renderer.cell_size, 30.0);
        
        // 別の値に変更
        renderer.set_cell_size(15.5);
        assert_eq!(renderer.cell_size, 15.5);
    }

    #[test]
    fn test_get_canvas_size() {
        // 2x3の配列でテスト
        let array = Array2::from_shape_vec((2, 3), vec![0, 1, 0, 1, 0, 1]).unwrap();
        let mut renderer = CanvasRenderer::new(array);
        
        // デフォルトのセルサイズ（20.0）での計算
        // 幅: 3列 * 20.0 = 60.0
        // 高さ: 2行 * 20.0 = 40.0
        let size = renderer.get_canvas_size();
        assert_eq!(size.x, 60.0);
        assert_eq!(size.y, 40.0);
        
        // セルサイズを変更してテスト
        renderer.set_cell_size(10.0);
        let size = renderer.get_canvas_size();
        assert_eq!(size.x, 30.0); // 3列 * 10.0
        assert_eq!(size.y, 20.0); // 2行 * 10.0
        
        // 小数点のセルサイズでテスト
        renderer.set_cell_size(15.5);
        let size = renderer.get_canvas_size();
        assert_eq!(size.x, 46.5); // 3列 * 15.5
        assert_eq!(size.y, 31.0); // 2行 * 15.5
    }

    #[test]
    fn test_get_canvas_size_single_cell() {
        // 1x1の配列でテスト
        let array = Array2::from_shape_vec((1, 1), vec![1]).unwrap();
        let renderer = CanvasRenderer::new(array);
        
        let size = renderer.get_canvas_size();
        assert_eq!(size.x, 20.0); // 1列 * 20.0
        assert_eq!(size.y, 20.0); // 1行 * 20.0
    }

    #[test]
    fn test_get_canvas_size_large_array() {
        // 大きな配列でテスト
        let data = vec![0; 100 * 50]; // 100行50列
        let array = Array2::from_shape_vec((100, 50), data).unwrap();
        let renderer = CanvasRenderer::new(array);
        
        let size = renderer.get_canvas_size();
        assert_eq!(size.x, 1000.0); // 50列 * 20.0
        assert_eq!(size.y, 2000.0); // 100行 * 20.0
    }

    #[test]
    fn test_new_renderer() {
        let array = Array2::from_shape_vec((3, 4), vec![0; 12]).unwrap();
        let renderer = CanvasRenderer::new(array.clone());
        
        assert_eq!(renderer.array, array);
        assert_eq!(renderer.cell_size, 20.0);
    }
}
