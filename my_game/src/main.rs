use my_game::MyGameApp;

fn main() -> eframe::Result<()> {
    let options = eframe::NativeOptions {
        viewport: eframe::egui::ViewportBuilder::default()
            .with_inner_size([800.0, 600.0])
            .with_title("2D Array Visualizer"),
        ..Default::default()
    };

    eframe::run_native(
        "My Game",
        options,
        Box::new(|_cc| Ok(Box::new(MyGameApp::default()))),
    )
}
