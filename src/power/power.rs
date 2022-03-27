use std::fs;
use std::path::Path;

use eframe::egui::{Grid, Slider};

use crate::settings::settings::Settings;

pub struct Power<'a> {
    name: &'a str,
    brightness: BrightNess,
}

#[derive(Debug)]
struct BrightNess {
    max_brightness: u8,
    brightness: u8,
    bright_device: String,
}

impl Settings for Power<'_> {
    fn name(&self) -> &str {
        self.name
    }

    fn show(&mut self, ui: &mut eframe::egui::Ui) {
        ui.heading(self.name());
        ui.separator();
        Grid::new("power_grid")
            .num_columns(2)
            .striped(true)
            .show(ui, |ui| {
                self.brightness.show(ui);
            });
    }

    fn apply(&mut self) {
        println!("Power apply");
        self.brightness.apply();
    }
}


impl Default for Power<'_> {
    fn default() -> Self {
        let brightness = BrightNess::new();
        Self {
            name: "Power Manager",
            brightness,
        }
    }
}

impl BrightNess {
    fn new() -> Self {
        // 1. get path
        let mut brightness_path = String::new();
        let entry = fs::read_dir("/sys/class/backlight/").unwrap();
        for entry in entry {
            let entry = entry.unwrap();
            let path = entry.path();
            if path.join("brightness").exists() {
                brightness_path = path.as_os_str().to_str().unwrap().to_string();
                break;
            }
        }
        // 2. get max_brightness
        let path = Path::new(&brightness_path);
        let max_brightness = Self::get_num_from_file(&path.join("max_brightness"));
        let brightness = Self::get_num_from_file(&path.join("brightness"));
        
        Self {
            max_brightness,
            brightness,
            bright_device: brightness_path,
        }
    }

    fn get_num_from_file(path: &Path) -> u8 {
        if !path.exists() {
            return 0;
        } 
        let data = fs::read_to_string(path).unwrap();
        data.trim().parse().unwrap_or(0)
    }
}

impl Settings for BrightNess {
    fn name(&self) -> &str {
        "BrightNess"
    }

    fn show(&mut self, ui: &mut eframe::egui::Ui) {
       ui.label("BrightNess");
       ui.add(Slider::new(&mut self.brightness, 1..=self.max_brightness));
       ui.end_row();
       println!("{:?}", self);
    }

    fn apply(&mut self) {
        let path = Path::new(&self.bright_device).join("brightness");
        fs::write(path.as_os_str().to_str().unwrap(), &self.brightness.to_string()).unwrap();
    }
}


