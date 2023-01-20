// hide console window on Windows in release
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use eframe::egui;
use egui::Color32;

fn main() {
    let options = eframe::NativeOptions {
        initial_window_size: Some(egui::vec2(640.0, 480.0)),
        ..Default::default()
    };
    eframe::run_native(
        "PainteRs",
        options,
        Box::new(|_cc| Box::new(PaintApp::default())),
    );
}

struct PaintImage {
    colorimage: egui::ColorImage,
    texture: Option<egui::TextureHandle>,
}
impl Default for PaintImage {
    fn default() -> Self {
        Self {
            colorimage: egui::ColorImage::new([400, 300], Color32::WHITE),
            texture: None,
        }
    }
}
impl PaintImage {
    fn get_texture(&mut self, ui: &mut egui::Ui) -> &egui::TextureHandle {
        self.texture.get_or_insert_with(|| {
            ui.ctx()
                .load_texture("paint-image", self.colorimage.clone(), Default::default())
        })
    }
    fn set_image(&mut self, dynamic_image: image::DynamicImage) {
        let size = [
            dynamic_image.width() as usize,
            dynamic_image.height() as usize,
        ];
        let image_buffer = dynamic_image.to_rgba8();
        let pixels = image_buffer.as_flat_samples();
        self.colorimage = egui::ColorImage::from_rgba_unmultiplied(size, pixels.as_slice());
        if let Some(texture_handle) = &mut self.texture {
            texture_handle.set(self.colorimage.clone(), egui::TextureOptions::default())
        };
    }
}

struct PaintApp {
    fpath: Option<String>,
    image: PaintImage,
}

impl Default for PaintApp {
    fn default() -> Self {
        Self {
            fpath: None,
            image: PaintImage::default(),
        }
    }
}

impl PaintApp {
    fn load_image(&mut self, fpath: impl Into<String>) {
        let s: String = fpath.into();
        let path = std::path::Path::new(&s);
        let dynamic_image = image::open(path).expect("unable to read file");
        self.image.set_image(dynamic_image);
        self.fpath = Some(s);
    }
}

impl eframe::App for PaintApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::TopBottomPanel::top("top_panel").show(ctx, |ui| {
            egui::menu::bar(ui, |ui| {
                ui.menu_button("File", |ui| {
                    if ui.button("Open file...").clicked() {
                        if let Some(fpath) = rfd::FileDialog::new().pick_file() {
                            self.load_image(fpath.display().to_string());
                            ui.close_menu();
                        }
                    }
                    if ui.button("Quit").clicked() {
                        _frame.close();
                    }
                });
            });
        });

        egui::TopBottomPanel::bottom("colors_panel").show(ctx, |ui| {
            ui.label("here will colors go");
        });
        egui::SidePanel::left("tool_panel").show(ctx, |ui| {
            ui.label("here will tools go");
        });

        egui::CentralPanel::default().show(ctx, |ui| {
            let texture = self.image.get_texture(ui);
            ui.image(texture, texture.size_vec2());
        });
    }
}
