// ui_theme.rs
use eframe::egui;

// 🌟 16進数から色を作る
pub fn hex(hex_str: &str) -> egui::Color32 {
    let r = u8::from_str_radix(&hex_str[1..3], 16).unwrap_or(0);
    let g = u8::from_str_radix(&hex_str[3..5], 16).unwrap_or(0);
    let b = u8::from_str_radix(&hex_str[5..7], 16).unwrap_or(0);
    egui::Color32::from_rgb(r, g, b)
}

// 🌟 ハイライトの設定を「装束」として管理する
pub struct 八百万の装束 {
    選択中の色: egui::Color32,
}

impl 八百万の装束 {
    pub fn new() -> Self {
        Self { 選択中の色: egui::Color32::WHITE }
    }

    pub fn set_color(&mut self, color: egui::Color32) {
        self.選択中の色 = color;
    }

    pub fn layout(&self, ui: &egui::Ui, text: &str) -> egui::text::LayoutJob {
        // 🌟 修正：ただ文字を描くのではなく、syntax.rs のハイライト機能を呼び出す
        // 繭さんが作った syntax.rs の関数をここで使います！
        crate::ui::syntax::highlight_yaoyorozu(ui, text)
    }
}

pub fn setup_custom_fonts(ctx: &egui::Context) {
    let mut fonts = egui::FontDefinitions::default();
    fonts.font_data.insert(
        "my_font".to_owned(),
        egui::FontData::from_static(include_bytes!("../assets/Noto_Sans_JP/NotoSansJP-VariableFont_wght.ttf")),
    );
    fonts.families.get_mut(&egui::FontFamily::Proportional).unwrap().insert(0, "my_font".to_owned());
    fonts.families.get_mut(&egui::FontFamily::Monospace).unwrap().push("my_font".to_owned());
    ctx.set_fonts(fonts);
}

pub fn apply_japanese_visuals(ctx: &egui::Context) {
    let mut visuals = egui::Visuals::light();
    visuals.window_fill = hex("#eae0d1"); // 生成り
    visuals.panel_fill = hex("#eae0d1");
    visuals.override_text_color = Some(hex("#2e3946")); // 青墨
    ctx.set_visuals(visuals);
}
