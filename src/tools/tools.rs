use std::sync::Mutex;
use std::sync::Arc;
use crate::settings::settings::Settings;
use eframe::egui::widgets::Spinner;
use eframe::egui::Grid;
use std::process::Command;
use std::sync::Condvar;
use std::thread;
use std::time::SystemTime;

#[derive(Default)]
pub struct Tools {
    screencast_cond: Arc<Condvar>,
    screencasting: bool,
    recordgif_cond: Arc<Condvar>,
    recordgifing: bool,
    init: bool,
}

impl Settings for Tools {
    fn name(&self) -> &str {
        "Tools"
    }

    fn show(&mut self, ui: &mut eframe::egui::Ui) {
        Grid::new("tools_grid").num_columns(2).show(ui, |ui| {
            // pick color
            ui.label("Get pix color");
            if ui.button("pick pix color").clicked() {
                self.pick_pix_color();
            }
            ui.end_row();
            // shotcut
            ui.label("Shotcut");
            if ui.button("shotcut").clicked() {
                self.shotcut();
            }
            ui.end_row();
            // recorder gif
            ui.label("Gif");
            if !self.recordgifing {
                if ui.button("gif").clicked() {
                    self.recordgifing = true;
                    self.record_gif();
                }
            } else {
                ui.add(Spinner::new());
                if ui.button("stop record gif").clicked() {
                    self.recordgifing = false;
                    self.stop_record_gif();
                }
            }
            ui.end_row();
            // recorder mp4
            ui.label("screnncast");
            if !self.screencasting {
            if ui.button("screnncast").clicked() {
                self.screencast();
                self.screencasting = true;
            }
            } else {
                ui.add(Spinner::new());
                if ui.button("stop screencast").clicked() {
                    self.screencasting = false;
                    self.stop_screencast();
                }
            }
        });
    }

    fn apply(&mut self) {
        todo!()
    }
    fn init(&mut self) {
        self.init = true;
    }

    fn is_init(&self) -> bool {
        self.init
    }
}

impl Tools {
    fn pick_pix_color(&self) {
        let cmd_str = r#"grim -g "$(slurp -p)" -t ppm - | convert - -format '%[pixel:p{0,0}]' txt:- | tail -n 1 | cut -d ' ' -f 4 | wl-copy"#;
        Command::new("zsh")
            .env("XCURSOR_SIZE", "48")
            .arg("-c")
            .arg(cmd_str)
            .output()
            .unwrap();
    }

    fn shotcut(&self) {
        let cmd_str = format!(
            r#"grim -g "$(slurp -p)" ~/temp/{}.png"#,
            Self::get_now_secs()
        );
        Command::new("zsh")
            .env("XCURSOR_SIZE", "48")
            .arg("-c")
            .arg(cmd_str.as_str())
            .output()
            .unwrap();
    }

    fn record_gif(&self) {
        let cmd_str = format!(
            r#"wf-recorder -f ~/temp/{}.gif -g "$(slurp)" -c gif"#,
            Self::get_now_secs()
        );
        let cond = self.recordgif_cond.clone();
        thread::spawn(move || {
            let mut cmd = Command::new("zsh");
                cmd.env("XCURSOR_SIZE", "48")
                .arg("-c")
                .arg(cmd_str.as_str());
            let mut child = cmd.spawn().unwrap();
            let guard = Arc::new(Mutex::new(1));
            let mut guard = guard.lock().unwrap();
            guard = cond.wait(guard).unwrap();
            child.kill().expect("Can't kill record gif");
        });
    }

    fn stop_record_gif(&self) {
        self.recordgif_cond.notify_one();
    }

    fn screencast(&self) {
        let cmd_str = format!(
            r#"wf-recorder -f ~/temp/{}.mkv -c libx264rgb"#,
            Self::get_now_secs()
        );
        let cond = self.screencast_cond.clone();
        thread::spawn(move || {
            let mut cmd = Command::new("zsh");
                cmd.env("XCURSOR_SIZE", "48")
                .arg("-c")
                .arg(cmd_str.as_str());
            let mut child = cmd.spawn().unwrap();
            let guard = Arc::new(Mutex::new(1));
            let guard = guard.lock().unwrap();
            cond.wait(guard).unwrap();
            child.kill().expect("Can't kill screencast");
        });
    }

    fn stop_screencast(&self) {
        self.screencast_cond.notify_one();
    }

    fn get_now_secs() -> u64 {
        let st = SystemTime::now();
        st.duration_since(SystemTime::UNIX_EPOCH).unwrap().as_secs()
    }
}

#[cfg(test)]
mod tests {
    use std::time::{self, SystemTime};

    #[test]
    fn time_test() {
        println!("sadsada");
        let st = time::SystemTime::now();
        println!(
            "{:?}",
            st.duration_since(SystemTime::UNIX_EPOCH).unwrap().as_secs()
        );
    }
}
