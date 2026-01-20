//Syntax---------->

use eframe::egui;
fn hex(hex_str: &str) -> egui::Color32 {
    let r = u8::from_str_radix(&hex_str[1..3], 16).unwrap_or(0);
    let g = u8::from_str_radix(&hex_str[3..5], 16).unwrap_or(0);
    let b = u8::from_str_radix(&hex_str[5..7], 16).unwrap_or(0);
    egui::Color32::from_rgb(r, g, b)
}
// src/ui/syntax.rs 

pub fn highlight_yaoyorozu(ui: &egui::Ui, code: &str) -> egui::text::LayoutJob {
   
    let mut job = egui::text::LayoutJob::default();

    // 🌟 12行目付近：自分を呼び出している不要な行があれば削除するか、
    // もし `text` という変数を使いたいなら `code` に書き換えます。

    // --- ここからハイライトのロジック ---
    let keywords = ["もし", "ならば", "または", "繰り返す", "｛", "｝"];
    let commands = ["表示", "待機", "取得"];

    for word in code.split_inclusive(|c: char| !c.is_alphanumeric() && c != '＿' && c != '※') {
        let color = if word.starts_with('※') {
            egui::Color32::from_gray(120) // 🌟 薄墨色（コメント用）
        } else if keywords.contains(&word.trim()) {
            hex("#B45064") // 苺色
        } else if commands.contains(&word.trim()) {
            hex("#98D98E") // 若緑
        } else if word.trim().parse::<f64>().is_ok() {
            hex("#478384") // 青碧
        } else {
            ui.visuals().strong_text_color() // 🌟 文字を見えやすく！
        };

        job.append(
            word,
            0.0,
            egui::TextFormat {
                font_id: egui::FontId::monospace(14.0),
                color,
                // 🌟 行間を 1.5倍（21.0px）に設定します
                line_height: Some(21.0),
                ..Default::default()
            },
        );
    }
    job
}