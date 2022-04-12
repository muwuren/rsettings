use crate::settings::settings::Settings;
use eframe::egui::{ComboBox, Grid};
use std::collections::BTreeMap;
use std::fs;
use std::path::Path;
use std::process::Command;

pub struct Appearance<'a> {
    name: &'a str,
    now: u8,
    themes: BTreeMap<u8, String>,
    init: bool,
}

impl Settings for Appearance<'_> {
    fn init(&mut self) {
        let appearance = Self::_init();
        (self.name, self.now, self.themes) = (appearance.name, appearance.now, appearance.themes);
        self.init = true;
    }

    fn is_init(&self) -> bool {
        return self.init;
    }

    fn name(&self) -> &str {
        self.name
    }
    fn show(&mut self, ui: &mut eframe::egui::Ui) {
        Grid::new("appearance_grid")
            .num_columns(2)
            .spacing([100.0, 8.0])
            .striped(true)
            .show(ui, |ui| {
                ui.label("GTK Theme");
                ComboBox::from_label("")
                    .selected_text(self.themes.get(&self.now).unwrap_or(&"".to_owned()))
                    .show_ui(ui, |ui| {
                        for (id, theme) in &self.themes {
                            ui.selectable_value(&mut self.now, *id, theme);
                        }
                    });
                ui.end_row();
                ui.collapsing("Theme", |ui| {
                    ui.label("sda");
                    ui.label("sda");
                });
            });
    }
    fn apply(&mut self) {
        println!("Appearance apply");
        if self.now == 0 {
            return;
        }
        let theme = self.themes.get(&self.now).unwrap();
        let _output = Command::new("gsettings")
            .args(["set", "org.gnome.desktop.interface", "gtk-theme"])
            .arg(theme.as_str())
            .output()
            .unwrap();
        let _output = Command::new("gsettings")
            .args(["set", "org.gnome.desktop.wm.preferences", "theme"])
            .arg(theme.as_str())
            .output()
            .unwrap();
    }
}

impl Default for Appearance<'_> {
    fn default() -> Self {
        Self {
            now: 0,
            name: "Appearance",
            themes: BTreeMap::new(),
            init: false,
        }
    }
}

impl Appearance<'_> {
    fn _init() -> Self {
        let mut appearance = Appearance::default();

        // 1. scan themes
        let dir = fs::read_dir("/usr/share/themes/").unwrap();
        let mut id = 1;
        let sys_theme = Self::get_system_gtk_theme().unwrap_or("".to_string());
        for entry in dir {
            let entry = entry.unwrap();
            let path = entry.path();

            if Self::is_complete_theme_dir(&path) {
                let theme = path.file_name().unwrap().to_str().unwrap().to_string();
                if theme == sys_theme {
                    appearance.now = id;
                }
                appearance.themes.insert(id, theme);
                id += 1;
            }
        }

        appearance
    }

    fn is_complete_theme_dir(path: &Path) -> bool {
        if !path.is_dir() {
            return false;
        }
        let files = vec!["gnome-shell", "xfwm4", "gtk-2.0", "gtk-3.0", "gtk-4.0"];
        for f in files {
            let path = path.join(f);
            if !path.exists() {
                return false;
            }
        }
        return true;
    }

    fn get_system_gtk_theme() -> Option<String> {
        let output = Command::new("gsettings")
            .args(["get", "org.gnome.desktop.interface", "gtk-theme"])
            .output()
            .unwrap();
        if !output.status.success() {
            return None;
        }
        match std::str::from_utf8(&output.stdout) {
            Ok(theme) => Some(theme.trim().replace("'", "").to_string()),
            Err(e) => {
                eprintln!("{}", e);
                None
            }
        }
    }
}
