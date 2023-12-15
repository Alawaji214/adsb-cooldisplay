use eframe::{egui, epi};
use egui::{paint::Shape, Color32, Frame, Stroke, Vec2};

struct App {
    // Your application state goes here...
}

impl epi::App for App {
    fn name(&self) -> &str {
        "EGUI display"
    }

    fn update(&mut self, ctx: &egui::CtxRef, _frame: &mut epi::Frame<'_>) {
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.heading("Cool!");

            // Draw the radar screen
            let (response, painter) = ui.allocate_painter(ui.available_size(), egui::Sense::hover());
            let center = response.rect.center();
            let radius = response.rect.width().min(response.rect.height()) / 2.0;
            let circle = Shape::circle_filled(center, radius, Color32::from_gray(128));
            let line = Shape::line_segment([center, egui::Pos2::new(center.x, center.y - radius).into()], Stroke::new(2.0, Color32::WHITE));
            // let line = Shape::line_segment([center, Vec2::new(center.x, center.y - radius)], Stroke::new(2.0, Color32::WHITE));
            painter.extend(vec![circle, line]);
        });
    }
}

fn main() {
    let app = App {
        // Initialize your application state here...
    };
    let native_options = eframe::NativeOptions::default();
    eframe::run_native(Box::new(app), native_options);
}