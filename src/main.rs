mod appearance;
mod display;
mod power;
mod settings;
mod network;

use crate::egui::{FontData, FontDefinitions, FontFamily};
use appearance::appearance::Appearance;
use eframe::epaint::Vec2;
use eframe::{egui, epi, NativeOptions};
use std::collections::BTreeMap;

struct MySettings {
    now: u8,
    labels: BTreeMap<u8, Box<dyn settings::settings::Settings>>,
}

impl epi::App for MySettings {
    fn name(&self) -> &str {
        "Settings for wayfire"
    }

    fn update(&mut self, ctx: &egui::Context, frame: &epi::Frame) {
        // side panel
        egui::SidePanel::left("left_panel")
            .max_width(40.0)
            .resizable(false)
            .show(ctx, |ui| {
                ui.add_space(5.5);
                ui.heading("Settings");
                ui.separator();
                egui::ScrollArea::vertical().show(ui, |ui| {
                    ui.vertical(|ui| {
                        for (id, label) in self.labels.iter_mut() {
                            ui.selectable_value(&mut self.now, *id, label.name());
                        }
                    });
                });
            });
        // center panel
        egui::CentralPanel::default().show(ctx, |ui| {
            egui::ScrollArea::new([true, true]).show(ui, |ui| {
                if self.now != 0 {
                    let f = self.labels.get_mut(&self.now).unwrap();
                    if !f.is_init() {
                        f.init();
                    }
                    ui.heading(f.heading());
                    ui.separator();
                    f.show(ui);
                }
            });
        });
        // bottom panel
        egui::TopBottomPanel::bottom("bottom")
            .resizable(false)
            .show(ctx, |ui| {
                if self.now != 0 {
                    ui.with_layout(egui::Layout::bottom_up(egui::Align::Max), |ui| {
                        ui.horizontal(|ui| {
                            if ui.button("Quit").clicked() {
                                frame.quit();
                            }
                            if ui.button("Apply").clicked() {
                                let f = self.labels.get_mut(&self.now).unwrap();
                                f.apply();
                            }
                        });
                    });
                }
            });
    }

    fn setup(
        &mut self,
        _ctx: &egui::Context,
        _frame: &epi::Frame,
        _storage: Option<&dyn epi::Storage>,
    ) {
        let mut fonts = FontDefinitions::default();
        fonts.font_data.insert("my_font".to_string(), FontData::from_static(include_bytes!("/home/nsfoxer/.local/share/fonts/PingFang.ttf")));
        fonts.families.get_mut(&FontFamily::Proportional).unwrap()
        .insert(0, "my_font".to_owned());
        _ctx.set_fonts(fonts);
        // 1. add displays
        let displays = display::display::Displays::default();
        self.add_label(1, Box::new(displays));
        // 2. add appearance
        let appearance = Appearance::default();
        self.add_label(2, Box::new(appearance));
        // 3. add power manager
        let power = power::power::Power::default();
        self.add_label(3, Box::new(power));
        // 4. add network
        let network = network::network::Network::default();
        self.add_label(4, Box::new(network));
    }
}

impl Default for MySettings {
    fn default() -> Self {
        Self {
            now: 0,
            labels: BTreeMap::<u8, Box<dyn settings::settings::Settings>>::new(),
        }
    }
}

impl MySettings {
    fn add_label(&mut self, key: u8, label: Box<dyn settings::settings::Settings>) {
        self.labels.insert(key, label);
    }
}

fn main() {
    let app = MySettings::default();
    let mut native_options = NativeOptions::default();
    native_options.initial_window_size = Some(Vec2::new(600.0, 400.0));
    native_options.min_window_size = Some(Vec2::new(300.0, 400.0));
    native_options.resizable = true;
    eframe::run_native(Box::new(app), native_options)
}
