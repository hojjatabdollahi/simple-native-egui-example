use eframe::{egui, epi};

struct TestApp {
    label: String,
}

impl Default for TestApp {
    fn default() -> Self {
        Self {
            label: "This is simple".to_owned(),
        }
    }
}

impl epi::App for TestApp {
    fn name(&self) -> &str {
        "Simple native example"
    }

    fn update(&mut self, ctx: &egui::CtxRef, _frame: &mut epi::Frame<'_>) {
        egui::CentralPanel::default().show(ctx, |ui|{
            ui.vertical_centered(|ui| {
                ui.add(egui::Label::new("Hello World!"));
                ui.separator();
                if ui.small_button("Click me!").clicked() {
                    self.label = "You clicked the button.".to_owned();
                }
                ui.label(&self.label);
            });
        });
    }
}

fn main() {
    let app = TestApp::default();
    let native_options = eframe::NativeOptions::default();
    eframe::run_native(Box::new(app), native_options);
}
