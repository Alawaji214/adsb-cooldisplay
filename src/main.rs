use eframe::{egui, epi};
use egui::{paint::Shape, Color32, Stroke};

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
            
            // Draw the three circles inner, middle
            let circle = Shape::circle_filled(center, radius, Color32::from_gray(32));
            painter.extend(vec![circle]);
            
            let inner_ring = Shape::circle_stroke(center, radius / 3.0, Stroke::new(2.0, Color32::LIGHT_GREEN));
            let middle_ring = Shape::circle_stroke(center, 2.0 * radius / 3.0, Stroke::new(2.0, Color32::LIGHT_GREEN));
            let outer_ring = Shape::circle_stroke(center, radius, Stroke::new(2.0, Color32::LIGHT_GREEN));
    
            painter.extend(vec![inner_ring, middle_ring, outer_ring]);
            
            // Draw the 8 lines at 45 degree intervals
            for i in 0..8 {
                let angle = i as f32 * std::f32::consts::PI / 4.0;
                let end_pos = egui::Pos2::new(center.x + radius * angle.cos(), center.y - radius * angle.sin());

                let line = Shape::line_segment([center, end_pos.into()], Stroke::new(2.0, Color32::LIGHT_GREEN));
                painter.add(line);
                
                // Show the angle in degrees as a label at the end of the line
                let text = format!("{:.0}°", angle.to_degrees());
                let text_pos = egui::Pos2::new(center.x + (radius * 1.1 ) * angle.cos(), center.y - (radius * 1.1 ) * angle.sin());
                painter.text(text_pos, egui::Align2::CENTER_CENTER, text, egui::TextStyle::Body, Color32::WHITE);

            }
            

            // let line = Shape::line_segment([center, egui::Pos2::new(center.x, center.y - radius).into()], Stroke::new(2.0, Color32::WHITE));
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