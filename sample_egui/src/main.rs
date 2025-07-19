use eframe::egui;
use egui::widgets;


#[derive(Default)]
struct MyEguiApp {}

impl MyEguiApp {
    fn new(cc: &eframe::CreationContext<'_>) -> Self {
        // Customize egui here with cc.egui_ctx.set_fonts and cc.egui_ctx.set_visuals.
        // Restore app state using cc.storage (requires the "persistence" feature).
        // Use the cc.gl (a glow::Context) to create graphics shaders and buffers that you can use
        // for e.g. egui::PaintCallback.
        Self::default()
    }
}

impl eframe::App for MyEguiApp {
   fn update(&mut self, ctx: &egui::Context, frame: &mut eframe::Frame) {
       egui::CentralPanel::default().show(ctx, |ui| {
           ui.heading("Hello World!");
           ui.add(widgets::Label::new("This is a simple egui app."));
              if ui.button("Click me!").clicked() {
               ui.label("Button clicked!");
           }
       });
   }
}

fn main() {
    let native_options = eframe::NativeOptions::default();
    let _ = eframe::run_native(
        "My egui App", 
        native_options, 
        Box::new(|cc| Ok(Box::new(MyEguiApp::new(cc))))
    );
}
