mod engine;

use eframe::egui;
use eframe::egui::{FontDefinitions, FontFamily, FontData};
use crate::engine::lexer::Lexer;
use crate::engine::parser::Parser;
use crate::engine::evaluator::Evaluator;

fn main() -> eframe::Result<()> {
    let options = eframe::NativeOptions::default();
    eframe::run_native(
        "八百万 (yaoyorozu) - 日本語エディタ",
        options,
        Box::new(|_cc| Box::new(YaoyorozuApp::default())),
    )
}

struct YaoyorozuApp {
    ソースコード: String,
    出力結果: String,
}

impl Default for YaoyorozuApp {
    fn default() -> Self {
        Self {
            ソースコード: "もし 10 ＝ 10 ならば ｛ 表示 100 ＋ 200 ｝".to_owned(),
            出力結果: "ここに結果が出ます".to_owned(),
        }
    }
}

impl eframe::App for YaoyorozuApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        // 日本語フォントの設定（毎回走りますが、一旦これでOK）
        let mut fonts = FontDefinitions::default();
        fonts.font_data.insert(
            "my_font".to_owned(),
            FontData::from_static(include_bytes!("C:/Windows/Fonts/msgothic.ttc")),
        );
        fonts.families.get_mut(&FontFamily::Proportional).unwrap()
            .insert(0, "my_font".to_owned());
        fonts.families.get_mut(&FontFamily::Monospace).unwrap()
            .push("my_font".to_owned());
        ctx.set_fonts(fonts);

        egui::CentralPanel::default().show(ctx, |ui| {
            ui.heading("八百万 (yaoyorozu) エディタ");

            ui.add_space(10.0);
            ui.label("プログラムを入力してください:");
            // 入力欄を大きく表示
            ui.add(egui::TextEdit::multiline(&mut self.ソースコード).desired_rows(10).desired_width(f32::INFINITY));

            ui.add_space(10.0);

            if ui.button("⚡ 実行する").clicked() {
                let レキシカ = Lexer::new(&self.ソースコード);
                let mut パーサ = Parser::new(レキシカ);
                let 構文木 = パーサ.解析する();
                
                let 実行機 = Evaluator::new();
                let 結果 = 実行機.実行(構文木);
                
                if 結果.is_empty() {
                    self.出力結果 = "（表示する結果がありませんでした）".to_owned();
                } else {
                    self.出力結果 = 結果;
                }
            }

            ui.add_space(20.0);
            ui.label("出力結果:");
            // 結果表示欄
            ui.code(&self.出力結果);
        });
    }
}