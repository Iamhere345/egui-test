use eframe::{run_native, App, NativeOptions};
use eframe::egui::{self, Align2, Color32, TextureOptions, Vec2};
use eframe::epaint::{ImageDelta, ColorImage};

use std::time::Instant;

struct EframeApp {
    last_update: Instant,
    frame_inc: u8,
}

impl EframeApp {
    fn new(cc: &eframe::CreationContext) -> Self {

        Self {
            last_update: Instant::now(),
            frame_inc: 0,
        }
    }
}

impl App for EframeApp {
    fn update(&mut self, ctx: &egui::Context, frame: &mut eframe::Frame) {

        egui::TopBottomPanel::top("Top pannel").show(ctx, |ui| {
            ui.horizontal(|ui| {
                if ui.button("button").clicked() {
                    println!("click.");
                }

                ui.label(format!("{} FPS", 1.0 / self.last_update.elapsed().as_secs_f64()));
            });

        });


        egui::SidePanel::left("Side bar").show(ctx, |ui| {

            for i in 0..10 {
                ui.label(format!("label {}", i));
            }

        });

        egui::Window::new("Canvas")
            .resizable(false)
            .collapsible(false)
            .anchor(Align2::CENTER_CENTER, [0.0, 0.0])
            .show(ctx, |ui| {
                let tex = ctx.load_texture("Image", ColorImage {
                    size: [480, 480],
                    pixels: vec![Color32::from_rgb(1 * self.frame_inc, 0, (1 * self.frame_inc) / 4); 480 * 480],
                }, TextureOptions::NEAREST);

                ui.image((tex.id(), Vec2 { x: 480.0, y: 480.0 }));
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

        self.frame_inc = self.frame_inc.overflowing_add(1).0;
        self.last_update = Instant::now();

        ctx.request_repaint();

    }
}

fn main() {
    let native_options = NativeOptions::default();

    run_native("Egui Test", native_options, Box::new(|cc| Box::new(EframeApp::new(cc)))).expect("Unable to initialise egui app");

}
