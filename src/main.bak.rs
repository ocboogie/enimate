use scene::Scene;

pub mod animation;
pub mod object;
pub mod scene;
pub mod world;

struct App {}

impl eframe::App for App {
    fn update(&mut self, ctx: &egui::Context, frame: &mut eframe::Frame) {}
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    tracing_subscriber::fmt::init();

    let native_options = eframe::NativeOptions::default();

    eframe::run_native(
        "My egui App",
        native_options,
        Box::new(|cc| Box::new(App {})),
    )?;

    Ok(())
}
