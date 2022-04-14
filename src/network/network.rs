use crate::settings::settings::Settings;

use eframe::egui;
use std::{process::Command, collections::HashSet};

#[derive(Default)]
pub struct Network {
    devices: Vec<Device>,
    live_wifis: Vec<Wifi>,
    known_wifis: HashSet<String>,
    current_wifi: String,
    init: bool,
}

#[derive(Default)]
struct Device {
    device: String,
    status: bool,
}

#[derive(Default)]
struct Wifi {
    bssid: String,
    ssid: String,
    mode: String,
    chan: u8,
    rate: String,
    signal: u8,
    bras: String,
    security: String,
}

impl Settings for Network {
    fn init(&mut self) {
        // 1. get devices
        self.get_devices();

        // 2. scan wifi and get wifi info
        self.scan_wifi();

        // 3. get known_wifis
        self.get_known_wifi();

        // 4. change init status
        self.init = true;
    }

    fn is_init(&self) -> bool {
        self.init
    }

    fn name(&self) -> &str {
        "Network"
    }

    fn apply(&mut self) {}

    fn show(&mut self, ui: &mut eframe::egui::Ui) {
        egui::Grid::new("network grid")
            .num_columns(3)
            .show(ui, |ui| {
                for device in self.devices.iter_mut() {
                    device.show(ui);
                }
            });
        ui.separator();
        egui::ScrollArea::both().show(ui, |ui| {
            egui::Grid::new("wifi").num_columns(6).show(ui, |ui| {
                for wifi in self.live_wifis.iter_mut() {
                    wifi.show(ui, &mut self.current_wifi);
                    ui.end_row();
                }
            });
        });
    }
}

impl Network {
    fn get_devices(&mut self) {
        let output = Command::new("nmcli")
            .args(["-t", "d"])
            .output()
            .expect("execute nmcli error");
        let output = String::from_utf8(output.stdout).unwrap();
        let mut outs = output.lines();
        while let Some(line) = outs.next() {
            let mut datas = line.split(':');
            let mut device = Device::default();
            device.device = datas.next().unwrap().to_string();
            device.status = true;
            self.devices.push(device);
        }
    }

    fn scan_wifi(&mut self) {
        // 1. scan wifi
        let output = Command::new("nmcli")
            .args(["-t", "device", "wifi", "list"])
            .output()
            .unwrap();
        let stdout = String::from_utf8(output.stdout).unwrap();
        let mut lines = stdout.lines();

        // 2. parser wifi
        while let Some(line) = lines.next() {
            let mut wifi = Wifi::default();
            // a. need to deal with bssid separately
            let line = line.replace("\\:", "-");
            let mut data = line.split(':');
            let current = data.next().unwrap();
            wifi.bssid = data.next().unwrap().replace('-', ":");
            wifi.ssid = data.next().unwrap().to_string();
            wifi.mode = data.next().unwrap().to_string();
            wifi.chan = data.next().unwrap().parse().unwrap();
            wifi.rate = data.next().unwrap().to_string();
            wifi.signal = data.next().unwrap().parse().unwrap();
            wifi.bras = data.next().unwrap().to_string();
            wifi.security = data.next().unwrap().to_string();

            // b. if current wifi
            if current == "*" {
                self.current_wifi = wifi.bssid.clone();
            }
            self.live_wifis.push(wifi);
        }
    }

    fn get_known_wifi(&mut self) {
        let output = Command::new("nmcli")
            .args(["-t", "connection", "show"])
            .output()
            .expect("nmcli execute error");
        let output = String::from_utf8(output.stdout).unwrap();
        let mut lines = output.lines();
        while let Some(line) = lines.next() {
            let mut data = line.split(";");
            self.known_wifis.insert(data.next().unwrap().to_string());
        }
    }
}

impl Device {
    fn show(&mut self, ui: &mut eframe::egui::Ui) {
        ui.label("Device");
        ui.label(self.device.as_str());
        ui.checkbox(&mut self.status, "enable");
        ui.end_row();
    }
}

impl Wifi {
    fn show(&mut self, ui: &mut eframe::egui::Ui, select: &mut String) {
        ui.radio_value(select, self.bssid.clone(), "");
        ui.label(&self.ssid);
        ui.label(&self.mode);
        ui.label(self.chan.to_string().as_str());
        ui.label(&self.rate);
        ui.label(self.signal.to_string().as_str());
        ui.label(&self.security);
    }
}
