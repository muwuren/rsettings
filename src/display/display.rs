use crate::settings::settings::Settings;

use eframe::egui::{self, ComboBox, Grid, Ui};
use regex::Regex;
use std::collections::BTreeMap;
use std::process::Command;
use std::str::Lines;

/// Display for display
pub struct Displays {
    displays: BTreeMap<String, Display>,
    now: String,
}

/// Display
#[derive(Default)]
struct Display {
    name: String,
    description: String,
    enable: bool,
    physical_size: String,
    mode: Vec<Resolution>,
    now_mode: Resolution,
    position: (u16, u16),
    transform: Transform,
    scale: f64,
}

#[derive(Default, Clone)]
struct Resolution {
    resolution: String,
    refresh: f64,
}

impl Settings for Displays {
    fn name(&self) -> &str {
        "Displays And Resolution"
    }
    fn show(&mut self, ui: &mut eframe::egui::Ui) {
        ui.heading("Resize and Roate display");
        ui.separator();
        Grid::new("display_grid")
            .num_columns(2)
            .spacing([100.0, 8.0])
            .striped(true)
            .show(ui, |ui| {
                ui.label("Display");
                ComboBox::from_label("")
                    .selected_text(self.now.as_str())
                    .show_ui(ui, |ui| {
                        for (name, _) in self.displays.iter() {
                            ui.selectable_value(&mut self.now, name.to_owned(), name.as_str());
                        }
                    });
                ui.end_row();
                let now_dis = self.displays.get_mut(&self.now).unwrap();
                now_dis.show(ui);
            });
    }
    fn apply(&mut self) {
        for (_, display) in &mut self.displays {
            display.apply();
        }
    }
}

impl Settings for Display {
    fn name(&self) -> &str {
        &self.name
    }
    fn show(&mut self, ui: &mut eframe::egui::Ui) {
        ui.label("Display Name");
        ui.label(&self.description);
        ui.end_row();
        ui.label("Physical Size");
        ui.label(&self.physical_size);
        ui.end_row();
        ui.label("Enable");
        ui.checkbox(&mut self.enable, "enable display");
        ui.end_row();
        ui.label("Transform");
        ComboBox::from_label("transform")
            .selected_text(self.transform.as_str())
            .show_ui(ui, |ui| {
                ui.selectable_value(&mut self.transform, Transform::Normal, "normal");
                ui.selectable_value(&mut self.transform, Transform::Roate90, "90");
                ui.selectable_value(&mut self.transform, Transform::Roate180, "180");
                ui.selectable_value(&mut self.transform, Transform::Roate270, "270");
                ui.selectable_value(&mut self.transform, Transform::Flipped, "flipped");
            });
        ui.end_row();
        ui.label("Resolution&Refresh");
        ui.horizontal(|ui| {
            ComboBox::from_label("")
                .selected_text(self.now_mode.resolution.as_str())
                .show_ui(ui, |ui| {
                    for res in &self.mode {
                        ui.selectable_value(
                            &mut self.now_mode.resolution,
                            res.resolution.to_owned(),
                            res.resolution.as_str(),
                        );
                    }
                });
            ComboBox::from_label("Hz")
                .selected_text(self.now_mode.refresh.to_string().as_str())
                .show_ui(ui, |ui| {
                    for res in &self.mode {
                        if res.resolution == self.now_mode.resolution {
                            ui.selectable_value(
                                &mut self.now_mode.refresh,
                                res.refresh,
                                res.refresh.to_string().as_str(),
                            );
                        }
                    }
                });
        });
        ui.end_row();
        ui.label("Position");
        ui.horizontal(|ui| {
            self.position.0 = num_edit(ui, self.position.0);
            ui.add_space(10.0);
            self.position.1 = num_edit(ui, self.position.1);
        });
        ui.end_row();
        ui.label("Scale");
        ui.add(egui::Slider::new(&mut self.scale, 0.1..=5.0));
        ui.end_row();
    }
    fn apply(&mut self) {
        println!("apply {}", self.name);
        let enable = if self.enable { "--on" } else { "--off" };
        let output = Command::new("wlr-randr")
            .arg("--output")
            .arg(&self.name)
            .arg("--mode")
            .arg(format!(
                "{}@{}",
                &self.now_mode.resolution, self.now_mode.refresh
            ))
            .arg("--scale")
            .arg(&self.scale.to_string())
            .arg("--transform")
            .arg(&self.transform.as_str())
            .arg(enable)
            .arg("--pos")
            .arg(&format!("{},{}", self.position.0, self.position.1))
            .output()
            .expect("Execute wlr-randr error");
        if !output.status.success() {
            eprintln!("wlr-randr execute error");
        }
    }
}

impl Default for Displays {
    fn default() -> Self {
        Self::init()
    }
}

impl Displays {
    fn init() -> Self {
        // 1. get display info
        let output = Command::new("wlr-randr").output().unwrap();
        let out = String::from_utf8(output.stdout).unwrap();
        let mut outs = out.lines();

        // 2. parser display info
        let mut now = String::new();
        let mut displays = BTreeMap::new();
        while let Some(line) = outs.next() {
            let display = Self::parser_display(&mut outs, line);
            now = display.name.to_owned();
            displays.insert(display.name.to_owned(), display);
        }

        Self { displays, now }
    }

    fn parser_display(outs: &mut Lines, first_line: &str) -> Display {
        // 1. parser name and description
        let mut display = Display::default();
        let datas: Vec<&str> = first_line.trim().split(' ').collect();
        let mut iter = datas.iter();
        let name = iter.next().unwrap();
        let mut description = String::new();
        for d in iter {
            description += d;
            description += " ";
        }
        display.name = name.to_string();
        display.description = description;

        // 2. parser physical_size
        let line = outs.next().unwrap();
        let datas: Vec<&str> = line.trim().split(':').collect();
        let physical_size = datas.get(1).unwrap();
        display.physical_size = physical_size.to_string();

        // 3. parser enable
        let line = outs.next().unwrap();
        let datas: Vec<&str> = line.trim().split(':').collect();
        let enable_s = datas.get(1).unwrap();
        let mut enable = false;
        if enable_s.trim() == "yes" {
            enable = true;
        }
        display.enable = enable;

        // 4. parser modes and position
        outs.next();
        let re = Regex::new(r"\s*(.*)\spx,\s(\d*\.\d*)\sHz").unwrap();
        loop {
            let line = outs.next().unwrap();
            if !line.contains("Hz") {
                let re = Regex::new(r"\s*:\s(\d+),(\d+)").unwrap();
                let caps = re.captures(line).unwrap();
                display.position = (
                    caps.get(1).unwrap().as_str().parse().unwrap(),
                    caps.get(2).unwrap().as_str().parse().unwrap(),
                );
                break;
            }
            let caps = re.captures(line).unwrap();
            println!("{:?}", caps);
            let mode = Resolution {
                resolution: caps.get(1).unwrap().as_str().to_string(),
                refresh: caps.get(2).unwrap().as_str().parse().unwrap(),
            };
            display.mode.push(mode.clone());
            if line.contains("current") {
                display.now_mode = mode;
            }
        }

        // 5. parser transform and scale
        let re = Regex::new(r"\s*:\s([\w|\.]*)").unwrap();
        let transform = re
            .captures(outs.next().unwrap())
            .unwrap()
            .get(1)
            .unwrap()
            .as_str();
        display.transform = Transform::from_str(transform).unwrap();
        display.scale = re
            .captures(outs.next().unwrap())
            .unwrap()
            .get(1)
            .unwrap()
            .as_str()
            .parse()
            .unwrap();
        display
    }
}

fn num_edit(ui: &mut Ui, num: u16) -> u16 {
    let mut s = num.to_string();
    let resp = ui.add(egui::TextEdit::singleline(&mut s).desired_width(100.0));
    if resp.changed() {
        let n = s.trim().parse::<u16>();
        if let Ok(n) = n {
            return n;
        }
    }
    num
}

#[derive(PartialEq)]
enum Transform {
    Normal,
    Roate90,
    Roate180,
    Roate270,
    Flipped,
}

impl Default for Transform {
    fn default() -> Self {
        Transform::Normal
    }
}

impl Transform {
    fn as_str(&self) -> &str {
        match self {
            Transform::Normal => "normal",
            Transform::Roate90 => "90",
            Transform::Roate180 => "180",
            Transform::Roate270 => "270",
            Transform::Flipped => "flipped",
        }
    }

    fn from_str(s: &str) -> Option<Self> {
        match s {
            "normal" => Some(Transform::Normal),
            "90" => Some(Transform::Roate90),
            "180" => Some(Transform::Roate180),
            "270" => Some(Transform::Roate270),
            "flipped" => Some(Transform::Flipped),
            _ => None,
        }
    }
}
