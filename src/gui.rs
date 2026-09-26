use std::time::Instant;

use anyhow::Result;
use eframe::{
    egui,
    epaint::text::{FontInsert, InsertFontFamily},
};

use crate::{actions::handle::handle_action, config::Config};

pub struct InfiltratorApp {
    config: Config,
    query: String,
    focus_set: bool,
    start_time: Instant,
}

impl InfiltratorApp {
    pub fn new(config: Config) -> Self {
        Self {
            config,
            query: String::new(),
            focus_set: false,
            start_time: Instant::now(),
        }
    }
}

impl eframe::App for InfiltratorApp {
    fn clear_color(&self, _visuals: &egui::Visuals) -> [f32; 4] {
        // Semi-transparent dark overlay
        egui::Rgba::from_rgba_unmultiplied(0.02, 0.02, 0.04, 0.25).to_array()
    }

    fn ui(&mut self, ctx: &mut egui::Ui, _frame: &mut eframe::Frame) {
        ctx.request_repaint();

        let elapsed = self.start_time.elapsed().as_secs_f32();

        let primary_rgb = self.config.theme.primary.unwrap_or([255, 15, 123]);
        let secondary_rgb = self.config.theme.secondary.unwrap_or([248, 155, 41]);

        // Close on 'Esc'
        if ctx.input(|i| i.key_pressed(egui::Key::Escape)) {
            ctx.send_viewport_cmd(egui::ViewportCommand::Close);
            return;
        }

        // Add font
        ctx.add_font(FontInsert::new(
            "google sans",
            egui::FontData::from_static(include_bytes!("../GoogleSansFlex_120pt-Regular.ttf")),
            vec![InsertFontFamily {
                family: egui::FontFamily::Proportional,
                priority: egui::epaint::text::FontPriority::Highest,
            }],
        ));

        // Replace fonts
        let mut fonts = egui::FontDefinitions::default();

        fonts.font_data.insert(
            "google sans".to_owned(),
            std::sync::Arc::new(egui::FontData::from_static(include_bytes!(
                "../GoogleSansFlex_120pt-Regular.ttf"
            ))),
        );

        fonts
            .families
            .entry(egui::FontFamily::Proportional)
            .or_default()
            .insert(0, "google sans".to_owned());

        ctx.set_fonts(fonts);

        // Fullscreen backdrop and waves
        egui::CentralPanel::default()
            .frame(egui::Frame::NONE.fill(egui::Color32::from_black_alpha(160)))
            .show(ctx, |ui| {
                let rect = ui.max_rect();
                let painter = ui.painter();

                // Gradient
                let pulse = (elapsed * 2.0).sin() * 0.15 + 0.85;
                let glow_c1 = egui::Color32::from_rgba_unmultiplied(
                    primary_rgb[0],
                    primary_rgb[1],
                    primary_rgb[2],
                    (45.0 * pulse) as u8,
                );
                let glow_c2 = egui::Color32::from_rgba_unmultiplied(
                    secondary_rgb[0],
                    secondary_rgb[1],
                    secondary_rgb[2],
                    (45.0 * pulse) as u8,
                );

                // top and bottom edge glow
                let edge_thickness = 180.0;

                paint_linear_glow(
                    painter,
                    egui::Rect::from_min_max(
                        rect.min,
                        egui::pos2(rect.max.x, rect.min.y + edge_thickness),
                    ),
                    glow_c2,
                    true,
                );
                paint_linear_glow(
                    painter,
                    egui::Rect::from_min_max(
                        egui::pos2(rect.min.x, rect.max.y - edge_thickness),
                        rect.max,
                    ),
                    glow_c1,
                    false,
                );

                // corner radial
                let cor_rad = 320.0;
                paint_radial_glow(painter, rect.min, cor_rad, glow_c1);
                paint_radial_glow(
                    painter,
                    egui::pos2(rect.max.x, rect.min.y),
                    cor_rad,
                    glow_c2,
                );
                paint_radial_glow(
                    painter,
                    egui::pos2(rect.min.x, rect.max.y),
                    cor_rad,
                    glow_c2,
                );
                paint_radial_glow(painter, rect.max, cor_rad, glow_c1);

                // Wave
                let wave_color =
                    egui::Color32::from_rgb(secondary_rgb[0], secondary_rgb[1], secondary_rgb[2]);
                let thick_wave_color = egui::Color32::from_rgba_unmultiplied(
                    primary_rgb[0],
                    primary_rgb[1],
                    primary_rgb[2],
                    45,
                );

                let wave_center_y = rect.center().y + 100.0;
                let mut wave_pts_main: Vec<egui::Pos2> = Vec::new();
                let mut wave_pts_glow: Vec<egui::Pos2> = Vec::new();

                for x in (0..rect.width() as i32).step_by(6) {
                    let x_f = x as f32;

                    // main wave
                    let y_main = wave_center_y
                        + (x_f * 0.012 + elapsed * 3.2).sin() * 14.0
                        + (x_f * 0.025 - elapsed * 2.1).cos() * 8.0;
                    wave_pts_main.push(egui::pos2(x_f, y_main));

                    // secondary wave
                    let y_glow = wave_center_y
                        + (x_f * 0.009 - elapsed * 2.5).cos() * 20.0
                        + (x_f * 0.018 + elapsed * 1.8).sin() * 10.0;
                    wave_pts_glow.push(egui::pos2(x_f, y_glow));
                }

                for window in wave_pts_glow.windows(2) {
                    painter.line_segment(
                        [window[0], window[1]],
                        egui::Stroke::new(8.0, thick_wave_color),
                    );
                }

                for window in wave_pts_main.windows(2) {
                    painter
                        .line_segment([window[0], window[1]], egui::Stroke::new(2.5, wave_color));
                }

                // Borderless text box
                let hud_width = 800.0;
                let hud_rect = egui::Rect::from_center_size(
                    egui::pos2(rect.center().x, rect.center().y - 40.0),
                    egui::vec2(hud_width, 160.0),
                );

                ui.scope_builder(egui::UiBuilder::new().max_rect(hud_rect), |ui| {
                    ui.vertical_centered(|ui| {
                        let text_edit = egui::TextEdit::singleline(&mut self.query)
                            .hint_text(
                                egui::RichText::new("Start typing...")
                                    .font(egui::FontId::proportional(60.0))
                                    .color(egui::Color32::from_rgba_unmultiplied(
                                        160, 160, 180, 120,
                                    )),
                            )
                            .font(egui::FontId::proportional(60.0))
                            .text_color(egui::Color32::from_rgb(240, 242, 255))
                            .horizontal_align(egui::Align::Center)
                            .frame(egui::Frame::NONE)
                            .desired_width(hud_width);

                        let response = ui.add(text_edit);

                        if !self.focus_set {
                            response.request_focus();
                            self.focus_set = true;
                        }

                        ui.add_space(40.0);

                        let keyword = self.query.trim().to_string();

                        if keyword.is_empty() {
                            ui.label(
                                egui::RichText::new("Press Esc to cancel").size(20.0).color(
                                    egui::Color32::from_rgba_unmultiplied(180, 180, 200, 140),
                                ),
                            );
                        } else if let Ok(action) = self.config.get_action(&keyword) {
                            ui.label(
                                egui::RichText::new(format!("Action: {:?}", action))
                                    .size(20.0)
                                    .color(egui::Color32::from_rgb(110, 235, 160)),
                            );

                            if response.lost_focus()
                                && ui.input(|i| i.key_pressed(egui::Key::Enter))
                            {
                                let action = action.clone();
                                let _ = handle_action(&action);
                                ui.ctx().send_viewport_cmd(egui::ViewportCommand::Close);
                            }
                        } else {
                            ui.label(
                                egui::RichText::new(format!(
                                    "No macro registered for '{}'",
                                    keyword
                                ))
                                .size(20.0)
                                .color(egui::Color32::from_rgb(240, 110, 110)),
                            );
                        }
                    });
                });
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
        "Infiltrator",
        options,
        Box::new(|_cc| Ok(Box::new(InfiltratorApp::new(config)))),
    )?;

    Ok(())
}

// Helpers
fn paint_radial_glow(
    painter: &egui::Painter,
    center: egui::Pos2,
    radius: f32,
    color: egui::Color32,
) {
    use egui::epaint::{Mesh, Shape, Vertex, WHITE_UV};
    let segments = 48;
    let mut mesh = Mesh::default();
    let edge_color = egui::Color32::from_rgba_unmultiplied(color.r(), color.g(), color.b(), 0);

    mesh.vertices.push(Vertex {
        pos: center,
        uv: WHITE_UV,
        color,
    });
    for i in 0..=segments {
        let angle = (i as f32 / segments as f32) * std::f32::consts::TAU;
        let pos = center + egui::vec2(angle.cos(), angle.sin()) * radius;
        mesh.vertices.push(Vertex {
            pos,
            uv: WHITE_UV,
            color: edge_color,
        });
    }
    for i in 1..=segments {
        mesh.indices.extend_from_slice(&[0, i as u32, i as u32 + 1]);
    }
    painter.add(Shape::mesh(mesh));
}

fn paint_linear_glow(
    painter: &egui::Painter,
    rect: egui::Rect,
    color: egui::Color32,
    fade_downward: bool,
) {
    use egui::epaint::{Mesh, Shape, Vertex, WHITE_UV};
    let mut mesh = Mesh::default();
    let transparent = egui::Color32::from_rgba_unmultiplied(color.r(), color.g(), color.b(), 0);
    let (top, bottom) = if fade_downward {
        (color, transparent)
    } else {
        (transparent, color)
    };

    mesh.vertices.push(Vertex {
        pos: rect.left_top(),
        uv: WHITE_UV,
        color: top,
    });
    mesh.vertices.push(Vertex {
        pos: rect.right_top(),
        uv: WHITE_UV,
        color: top,
    });
    mesh.vertices.push(Vertex {
        pos: rect.right_bottom(),
        uv: WHITE_UV,
        color: bottom,
    });
    mesh.vertices.push(Vertex {
        pos: rect.left_bottom(),
        uv: WHITE_UV,
        color: bottom,
    });
    mesh.indices.extend_from_slice(&[0, 1, 2, 0, 2, 3]);
    painter.add(Shape::mesh(mesh));
}
