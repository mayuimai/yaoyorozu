use eframe::egui;

pub fn setup_custom_fonts(ctx: &egui::Context) {
    let mut fonts = egui::FontDefinitions::default();
    fonts.font_data.insert(
        "my_font".to_owned(),
        egui::FontData::from_static(include_bytes!("C:/Windows/Fonts/msgothic.ttc")),
    );
    fonts.families.get_mut(&egui::FontFamily::Proportional).unwrap().insert(0, "my_font".to_owned());
    fonts.families.get_mut(&egui::FontFamily::Monospace).unwrap().push("my_font".to_owned());
    ctx.set_fonts(fonts);
}

pub fn apply_japanese_visuals(ctx: &egui::Context) {
    let mut visuals = egui::Visuals::light(); 
    visuals.window_fill = egui::Color32::from_rgb(247, 245, 237); // 生成り
    visuals.panel_fill = egui::Color32::from_rgb(247, 245, 237);
    visuals.override_text_color = Some(egui::Color32::from_rgb(39, 49, 62)); // 青墨
    ctx.set_visuals(visuals);
}