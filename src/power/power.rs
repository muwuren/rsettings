use std::fs;
use std::path::Path;

use eframe::egui::{ComboBox, Grid, Slider, TextBuffer, Ui};

use crate::settings::settings::Settings;

pub struct Power<'a> {
    name: &'a str,
    brightness: BrightNess,
    lenovo: LenovoBattery,
    cpufreq: CPUFreq,
}

///cpu freq
#[derive(PartialEq)]
enum CPUFreq {
    Performance,
    Powersave,
    Userspace,
    Ondemand,
    Conservative,
    Schedutil,
    Unknown,
}

impl CPUFreq {
    fn as_str(&self) -> &str {
        match self {
            Self::Performance => "performance",
            Self::Powersave => "powersave",
            Self::Userspace => "userspace",
            Self::Ondemand => "Ondemand",
            Self::Conservative => "conservative",
            Self::Schedutil => "schedutil",
            Self::Unknown => "unknown",
        }
    }

    fn from_str(governor: &str) -> Self {
        match governor {
            "performance" => CPUFreq::Performance,
            "powersave" => CPUFreq::Powersave,
            "userspace" => CPUFreq::Userspace,
            "ondemand" => CPUFreq::Ondemand,
            "conservative" => CPUFreq::Conservative,
            "schedutil" => CPUFreq::Schedutil,
            _ => CPUFreq::Unknown,
        }
    }

    fn new() -> Self {
        let s =
            fs::read_to_string("/sys/devices/system/cpu/cpu0/cpufreq/scaling_governor").unwrap();
        Self::from_str(s.trim().as_str())
    }

    fn show_ui(&mut self, ui: &mut Ui) {
        ui.label("cpu freq")
            .on_hover_text("Need root change power saving mode!");
        ui.add_enabled_ui(false, |ui| {
            ComboBox::from_label("select cpu freq")
                .selected_text(self.as_str())
                .show_ui(ui, |ui| {
                    ui.selectable_value(self, Self::Performance, Self::Performance.as_str());
                    ui.selectable_value(self, Self::Powersave, Self::Powersave.as_str());
                    ui.selectable_value(self, Self::Schedutil, Self::Schedutil.as_str());
                });
        });
        ui.end_row();
    }
}

struct LenovoBattery {
    saving: bool,
    lenovo: bool,
}
impl LenovoBattery {
    fn _is_lenovo() -> bool {
        let path = Path::new("/sys/bus/platform/drivers/ideapad_acpi/VPC2004:00/conservation_mode");
        path.exists()
    }

    fn new() -> Self {
        let mut saving = false;
        let lenovo = LenovoBattery::_is_lenovo();
        if lenovo {
            let status: u8 = fs::read_to_string(
                "/sys/bus/platform/drivers/ideapad_acpi/VPC2004:00/conservation_mode",
            )
            .unwrap()
            .trim()
            .parse()
            .unwrap();
            if status == 1 {
                saving = true;
            }
        }
        Self { saving, lenovo }
    }

    fn show_ui(&mut self, ui: &mut Ui) {
        if !self.lenovo {
            return;
        }
        ui.label("LenovoBattery")
            .on_hover_text("Need root change power saving mode!");
        ui.add_enabled_ui(false, |ui| {
            ui.checkbox(&mut self.saving, "power saving");
        });
        ui.end_row();
    }
}

#[derive(Debug)]
struct BrightNess {
    max_brightness: u8,
    percent: u8,
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
            .spacing([100.0, 8.0])
            .striped(true)
            .show(ui, |ui| {
                self.brightness.show(ui);
                self.lenovo.show_ui(ui);
                self.cpufreq.show_ui(ui);
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
        let lenovo = LenovoBattery::new();
        let cpufreq = CPUFreq::new();
        Self {
            name: "Power Manager",
            brightness,
            lenovo,
            cpufreq,
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
        let percent = ((brightness as f64 / max_brightness as f64) * 100f64) as u8;

        Self {
            max_brightness,
            percent,
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
        ui.add(Slider::new(&mut self.percent, 1..=100));
        ui.end_row();
    }

    fn apply(&mut self) {
        // 1. calculate brightness
        let brightness = ((self.max_brightness as f64 / 100.0) * self.percent as f64) as u8;
        let path = Path::new(&self.bright_device).join("brightness");
        fs::write(path.as_os_str().to_str().unwrap(), &brightness.to_string()).unwrap();
    }
}
