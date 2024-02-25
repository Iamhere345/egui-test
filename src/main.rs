use eframe::{run_native, App, NativeOptions};
use eframe::egui;
use eframe::epaint::{ImageDelta, ColorImage};

#[derive(Default)]
struct EframeApp;

impl EframeApp {
    fn new(cc: &eframe::CreationContext) -> Self {

        Self::default()
    }
}

impl App for EframeApp {
    fn update(&mut self, ctx: &egui::Context, frame: &mut eframe::Frame) {

        egui::TopBottomPanel::top("Top pannel").show(ctx, |ui| {
            ui.horizontal(|ui| {
                if ui.button("button").clicked() {
                    println!("click.");
                }

                ui.label("<- a button");
            });

        });


        egui::SidePanel::left("Side bar").show(ctx, |ui| {

            for i in 0..10 {
                ui.label(format!("label {}", i));
            }

        });

        egui::Window::new("Window").resizable(true).show(ctx, |ui| {

            ui.horizontal_top(|ui| {
                ui.button("File");
                ui.button("Edit");
                ui.button("Options");
                ui.button("Window");
                ui.button("Help");
            });

            ui.vertical(|ui| {
                ui.hyperlink("https://github.com/Iamhere345");
            })

        });

        egui::CentralPanel::default().show(ctx, |ui| {
            ui.heading("Hello, World!");

            
        });
    }
}

fn main() {
    let native_options = NativeOptions::default();

    run_native("Egui Test", native_options, Box::new(|cc| Box::new(EframeApp::new(cc)))).expect("Unable to initialise egui app");

}