//-----読み込み系モジュール-----------//
// 1. まずは「材料」や「設定」
    mod engine;    // 翻訳エンジン
    mod ui_theme;  // 見た目の設定

// 2. 次に「画面の部品」
    mod ui;        // サイドバーなど

// 3. 最後にそれらをまとめる「心臓部」
    mod app;       // アプリ全体の統合
//-----------------------------------//

use app::YaoyorozuApp;
use eframe::egui;

fn main() -> eframe::Result<()> {
    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default()
            .with_inner_size([1000.0, 700.0])
            .with_decorations(false), // OSの枠を消す
        ..Default::default()
    };
    
    eframe::run_native(
        "八百万エディタ",
        options,
        Box::new(|cc| Ok(Box::new(YaoyorozuApp::new(cc)))),
    )
}