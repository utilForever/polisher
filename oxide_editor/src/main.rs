mod app;
mod components;
mod models;
mod viewmodels;
mod views;

fn main() {
    let app = app::TemplateApp::default();
    let native_options = eframe::NativeOptions {
        maximized: true,
        ..Default::default()
    };
    eframe::run_native(Box::new(app), native_options);
}
