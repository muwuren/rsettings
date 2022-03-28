use std::collections::BTreeMap;
use std::fs;
use std::path::Path;
use std::process::Command;
use crate::settings::settings::Settings;
use eframe::egui::{ComboBox, Grid};

pub struct Appearance<'a> {
    name: &'a str,
    now: u8,
    themes: BTreeMap<u8, String>,
}

impl Settings for Appearance<'_> {
    fn name(&self) -> &str {
        self.name
    }
    fn show(&mut self, ui: &mut eframe::egui::Ui) {
        ui.heading("Appearance");
        ui.separator();
        Grid::new("appearance_grid")
            .num_columns(2)
            .spacing([100.0, 8.0])
            .striped(true)
            .show(ui, |ui| {
                ui.label("GTK Theme");
                ComboBox::from_label("take gtk theme")
                    .selected_text(&Appearance::get_system_gtk_theme().unwrap_or("".to_string()))
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
        let output = Command::new("gsettings")
            .args(["set", "org.gnome.desktop.interface", "gtk-theme"])
            .arg(theme.as_str())
            .output()
            .unwrap();
        let output = Command::new("gsettings")
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
        }
    }
}

impl Appearance<'_> {
    pub fn init() -> Self {
        let mut appearance = Appearance::default();

        // 1. scan themes
        let dir = fs::read_dir("/usr/share/themes/").unwrap();
        let mut id = 1;
        for entry in dir {
            let entry = entry.unwrap();
            let path = entry.path();

            if Self::is_complete_theme_dir(&path) {
                appearance.themes.insert(id, path.file_name().unwrap().to_str().unwrap().to_string());
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

    fn get_system_gtk_theme() -> Option<String>{
        let output = Command::new("gsettings")
            .args(["get", "org.gnome.desktop.interface", "gtk-theme"])
            .output().unwrap();
        if !output.status.success() {
            return None
        }
        match std::str::from_utf8(&output.stdout) {
            Ok(theme) => Some(theme.trim().replace("'", "").to_string()),
            Err(e) => {
                eprintln!("{}", e);
                None
            },
        }
    }
    
    pub fn check_need_init(&self) -> bool {
        if self.now == 0 {
            return true;
        }
        return false;
    }
}
