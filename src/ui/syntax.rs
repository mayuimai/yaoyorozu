//Syntax---------->

use eframe::egui;

pub fn highlight_yaoyorozu(ui: &egui::Ui, code: &str) -> egui::text::LayoutJob {
    let mut job = egui::text::LayoutJob::default();

    // 八百万のキーワード定義
    let keywords = ["もし", "ならば", "または", "繰り返す", "｛", "｝"];
    let commands = ["表示", "待機", "取得"];

    for word in code.split_inclusive(|c: char| !c.is_alphanumeric() && c != '＿') {
        let color = if keywords.contains(&word.trim()) {
            egui::Color32::from_rgb(180, 80, 100)  // 苺色（いちごいろ）: 落ち着いたピンク
        } else if commands.contains(&word.trim()) {
            egui::Color32::from_rgb(152, 217, 142)  // 若緑: 落ち着いた青
        } else if word.trim().parse::<f64>().is_ok() {
            egui::Color32::from_rgb(71, 131, 132)    // 青碧: 渋い緑
        } else {
            ui.visuals().widgets.noninteractive.text_color() // 通常文字
        };

        job.append(
            word,
            0.0,
            egui::TextFormat {
                font_id: egui::FontId::monospace(14.0),
                color,
                ..Default::default()
            },
        );
    }
    job
}