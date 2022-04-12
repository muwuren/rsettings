use crate::settings::settings::Settings;

#[derive(Default)]
pub struct Network {
    devices: Vec<String>,
    live_wifis: Vec<String>,
    known_wifis: Vec<String>,
}

impl Settings for Network {
    fn name(&self) -> &str {
        "Network"
    }

    fn apply(&mut self) {
        
    }

    fn show(&mut self, ui: &mut eframe::egui::Ui) {
        
    }
}

impl Network {
    fn init(&mut self) {
        
    }
}
