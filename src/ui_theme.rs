// ui_theme.rs
use eframe::egui;

// 🌟 ここにも魔法の道具（hex関数）を置いておきます
fn hex(hex_str: &str) -> egui::Color32 {
    let r = u8::from_str_radix(&hex_str[1..3], 16).unwrap_or(0);
    let g = u8::from_str_radix(&hex_str[3..5], 16).unwrap_or(0);
    let b = u8::from_str_radix(&hex_str[5..7], 16).unwrap_or(0);
    egui::Color32::from_rgb(r, g, b)
}

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
    // 🌟 hex関数を使って指定できるようになりました！
    visuals.window_fill = hex("#eae0d1"); // 生成り（きなり）
    visuals.panel_fill = hex("#eae0d1");
    visuals.override_text_color = Some(hex("#2e3946")); // 青墨（あおずみ）
    ctx.set_visuals(visuals);
}