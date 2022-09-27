use egui::{Button, FontData, FontDefinitions, FontFamily, Label, Response, RichText, Widget};

pub mod icons {
    material_icons_proc_macro::code_points!();
}

const FONT_DATA: &[u8] = include_bytes!("../../MaterialIcons-Regular.ttf");

pub fn initialize(ctx: &egui::Context) {
    let mut fonts = FontDefinitions::default();
    let data = FontData::from_static(FONT_DATA);
    fonts.font_data.insert("material-icons".to_string(), data);
    fonts.families.get_mut(&FontFamily::Proportional).unwrap().push("material-icons".to_string());

    ctx.set_fonts(fonts);
}

pub fn icon_button(ui: &mut egui::Ui, icon: &str) -> Response {
    Button::new(RichText::new(icon).size(20.0))
        .frame(false)
        .ui(ui)
}
