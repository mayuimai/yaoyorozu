#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")] // リリース時に黒い画面を消すおまじない

// 📂 作ったファイルたちを登録します
pub mod engine;
pub mod ui;
pub mod ui_theme;
pub mod app;

use eframe::egui;

fn main() -> eframe::Result<()> {
    // ウィンドウの設定
    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default()
            .with_inner_size([1200.0, 800.0]) // Arcっぽく横長に広く
            .with_title("八百万 (Yaoyorozu)"),
        ..Default::default()
    };

    // アプリを起動！
    eframe::run_native(
        "八百万",
        options,
        Box::new(|cc| Ok(Box::new(app::YaoyorozuApp::new(cc)))),
    )
}