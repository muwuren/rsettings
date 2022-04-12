use crate::egui::Ui;

pub trait Settings {
    fn is_init(&self) -> bool {
        return false;
    }
    fn init(&mut self);
    fn show(&mut self, ui: &mut Ui);
    fn name(&self) -> &str;
    fn heading(&self) -> &str {
        self.name()
    }
    fn apply(&mut self);
}
