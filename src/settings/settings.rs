use crate::egui::Ui;

pub trait Settings {
    fn show(&mut self, ui: &mut Ui);
    fn name(&self) -> &str;
    fn apply(&mut self);
}
