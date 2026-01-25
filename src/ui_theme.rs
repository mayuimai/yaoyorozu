use eframe::egui;

pub fn hex(hex_str: &str) -> egui::Color32 {
    let r = u8::from_str_radix(&hex_str[1..3], 16).unwrap_or(0);
    let g = u8::from_str_radix(&hex_str[3..5], 16).unwrap_or(0);
    let b = u8::from_str_radix(&hex_str[5..7], 16).unwrap_or(0);
    egui::Color32::from_rgb(r, g, b)
}

pub struct 八百万の装束 {
    選択中の色: egui::Color32,
}

impl 八百万の装束 {
    pub fn new() -> Self {
        Self { 選択中の色: egui::Color32::WHITE }
    }
    
    // 🌟 修正箇所：LayoutJob を Arc<Galley> に変換！
    pub fn layout(&self, ui: &egui::Ui, text: &str, _wrap_width: f32) -> std::sync::Arc<egui::Galley> {
        let job = crate::ui::syntax::highlight_yaoyorozu(ui, text);
        ui.fonts(|f| f.layout_job(job))
    }
}

pub fn setup_custom_fonts(ctx: &egui::Context) {
    let mut fonts = egui::FontDefinitions::default();
    if let Ok(data) = std::fs::read("C:\\Windows\\Fonts\\msgothic.ttc") {
        fonts.font_data.insert(
            "my_font".to_owned(),
            egui::FontData::from_owned(data).tweak(egui::FontTweak { scale: 1.2, ..Default::default() }),
        );
        fonts.families.entry(egui::FontFamily::Proportional).or_default().insert(0, "my_font".to_owned());
        fonts.families.entry(egui::FontFamily::Monospace).or_default().insert(0, "my_font".to_owned());
    }
    ctx.set_fonts(fonts);
}

pub fn apply_japanese_visuals(ctx: &egui::Context) {
    let mut visuals = egui::Visuals::light();
    visuals.window_fill = hex("#eae0d1");
    visuals.panel_fill = hex("#eae0d1");
    visuals.override_text_color = Some(hex("#2e3946"));
    visuals.window_rounding = egui::Rounding::same(12.0);
    visuals.widgets.noninteractive.rounding = egui::Rounding::same(8.0);
    ctx.set_visuals(visuals);
}