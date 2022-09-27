use eframe::egui;
use egui::{FontId, Label, Widget, WidgetText};
use egui::WidgetText::RichText;
use egui_material_icons::{icon_button, initialize};
use egui_material_icons::icons::*;
use egui_demo_lib::DemoWindows;

fn main() {
    let native_options = eframe::NativeOptions::default();
    eframe::run_native("My egui App", native_options, Box::new(|cc| Box::new(MyEguiApp::new(cc))));
}

#[derive(Default)]
struct MyEguiApp {
    demo: DemoWindows,
}

impl MyEguiApp {
    fn new(cc: &eframe::CreationContext<'_>) -> Self {
        initialize(&cc.egui_ctx);

        Self::default()
    }
}

impl eframe::App for MyEguiApp {
    fn update(&mut self, ctx: &egui::Context, frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {

            ui.horizontal(|ui| {
                icon_button(ui, ICON_ADD);
                icon_button(ui, ICON_REMOVE);
                icon_button(ui, ICON_IMAGE);
            });

            //self.demo.ui(ctx);
        });
    }
}
