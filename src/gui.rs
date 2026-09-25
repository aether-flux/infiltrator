use anyhow::Result;
use eframe::egui;

use crate::{actions::handle::handle_action, config::Config};

pub struct InfiltratorApp {
    config: Config,
    query: String,
    focus_set: bool,
}

impl InfiltratorApp {
    pub fn new(config: Config) -> Self {
        Self {
            config,
            query: String::new(),
            focus_set: false,
        }
    }
}

impl eframe::App for InfiltratorApp {
    fn clear_color(&self, _visuals: &egui::Visuals) -> [f32; 4] {
        // Semi-transparent dark overlay (75% opacity)
        egui::Rgba::from_rgba_unmultiplied(0.02, 0.02, 0.04, 0.75).to_array()
    }

    fn ui(&mut self, ui: &mut egui::Ui, _frame: &mut eframe::Frame) {
        // Close on 'Esc'
        if ui.input(|i| i.key_pressed(egui::Key::Escape)) {
            ui.send_viewport_cmd(egui::ViewportCommand::Close);
            return;
        }

        let mut visuals = egui::Visuals::dark();
        visuals.window_fill = egui::Color32::from_rgb(20, 20, 26);
        visuals.window_corner_radius = egui::CornerRadius::same(16);
        visuals.window_stroke = egui::Stroke::new(1.0, egui::Color32::from_rgb(60, 60, 75));
        ui.set_visuals(visuals);

        // Center HUD window
        egui::Window::new("Infiltrator")
            .title_bar(false)
            .resizable(false)
            .collapsible(false)
            .anchor(egui::Align2::CENTER_CENTER, [0.0, 0.0])
            .fixed_size([600.0, 110.0])
            .show(ui, |nui| {
                nui.add_space(8.0);

                // Textbox
                let text_edit = egui::TextEdit::singleline(&mut self.query)
                    .hint_text("Type macro...")
                    .font(egui::FontId::proportional(22.0))
                    .desired_width(560.0)
                    .margin(egui::vec2(12.0, 10.0));

                let response = nui.add(text_edit);

                // Auto-focus input on first render frame
                if !self.focus_set {
                    response.request_focus();
                    self.focus_set = true;
                }

                nui.add_space(10.0);

                let keyword = self.query.trim();
                if keyword.is_empty() {
                    nui.label(
                        egui::RichText::new(
                            "Type a macro keyword to execute (Press Esc to cancel)",
                        )
                        .size(13.0)
                        .color(egui::Color32::from_rgb(130, 130, 140)),
                    );
                } else if let Ok(action) = self.config.get_action(keyword) {
                    nui.label(
                        egui::RichText::new(format!("Matched Action: {:?}", action))
                            .size(13.0)
                            .color(egui::Color32::from_rgb(80, 220, 130)),
                    );

                    // Execute action on 'Enter'
                    if response.lost_focus() && ui.input(|i| i.key_pressed(egui::Key::Enter)) {
                        let _ = handle_action(action);
                        ui.send_viewport_cmd(egui::ViewportCommand::Close);
                    }
                } else {
                    nui.label(
                        egui::RichText::new(format!("No macro registered for '{}'", keyword))
                            .size(13.0)
                            .color(egui::Color32::from_rgb(220, 90, 90)),
                    );
                }
            });
    }
}

pub fn run_overlay(config: Config) -> Result<()> {
    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default()
            .with_decorations(false)
            .with_transparent(true)
            .with_fullscreen(true)
            .with_always_on_top(),
        ..Default::default()
    };

    eframe::run_native(
        "infiltrator",
        options,
        Box::new(|_cc| Ok(Box::new(InfiltratorApp::new(config)))),
    )?;

    Ok(())
}
