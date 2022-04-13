use crate::settings::settings::Settings;

use std::process::Command;
use eframe::egui;

#[derive(Default)]
pub struct Network {
    devices: Vec<String>,
    live_wifis: Vec<Wifi>,
    known_wifis: Vec<String>,
    current_wifi: String,
    init:bool,
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
        let output = Command::new("nmcli").arg("-t d").output().expect("execute nmcli error");
        let output = String::from_utf8(output.stdout).unwrap();
        println!("{}", &output);
        let mut outs = output.lines();
        while let Some(line) = outs.next() {
            let mut datas = line.split(':');
            self.devices.push(datas.next().unwrap().to_string());
        }
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

    fn apply(&mut self) {
    }

    fn show(&mut self, ui: &mut eframe::egui::Ui) {
        egui::Grid::new("network grid")
            .num_columns(2)
            .show(ui, |ui| {
                for device in self.devices.iter() {
                    ui.label("Device");
                    ui.label(device.as_str());
                    ui.end_row();
                }
            });
        ui.separator();
        
    }
}

impl Network {
    fn scan_wifi(&mut self) {
        // 1. scan wifi
        let output = Command::new("nmcli").args(["-t", "device", "wifi", "list"]).output().unwrap();
        println!("{:?}", output);
        let stdout = String::from_utf8(output.stdout).unwrap();
        let mut lines =stdout.lines();

        // 2. parser wifi
        while let Some(line) = lines.next() {
            let mut wifi = Wifi::default();
            // a. need to deal with bssid separately
            let line = line.replace("\\:", "-");
            let mut data = line.split(':');
            let current = data.next().unwrap();
            wifi.bssid = data.next().unwrap().replace('-',":");
            wifi.ssid  = data.next().unwrap().to_string();
            wifi.mode  = data.next().unwrap().to_string();
            wifi.chan = data.next().unwrap().parse().unwrap();
            wifi.rate = data.next().unwrap().to_string();
            wifi.signal = data.next().unwrap().parse().unwrap();
            wifi.bras = data.next().unwrap().to_string();
            wifi.security = data.next().unwrap().to_string();
            
            // b. if current wifi
            if current == "*" {
                self.current_wifi = wifi.ssid.clone();
            }
            self.live_wifis.push(wifi);
        }
    }

    fn get_known_wifi(&mut self) {
        let output = Command::new("nmcli").args(["-t", "connection", "show"]).output().expect("nmcli execute error");
        let output = String::from_utf8(output.stdout).unwrap();
        println!("{}", output.as_str());
        let mut lines = output.lines();
        while let Some(line) = lines.next() {
            let mut data = line.split(";");
            self.known_wifis.push(data.next().unwrap().to_string());
        }
    }
}

impl Wifi {
}
