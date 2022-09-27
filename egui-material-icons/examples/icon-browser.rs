use eframe::egui;


use egui_material_icons::{icon_button, initialize};
use egui_material_icons::icons::*;

fn main() {
    let native_options = eframe::NativeOptions::default();
    eframe::run_native("My egui App", native_options, Box::new(|cc| Box::new(MyEguiApp::new(cc))));
}

#[derive(Default)]
struct MyEguiApp {
}

impl MyEguiApp {
    fn new(cc: &eframe::CreationContext<'_>) -> Self {
        initialize(&cc.egui_ctx);

        Self::default()
    }
}

impl eframe::App for MyEguiApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.horizontal(|ui| {
                icon_button(ui, ICON_ADD);
                icon_button(ui, ICON_REMOVE);
                icon_button(ui, ICON_IMAGE);
            });
        });
    }
}
