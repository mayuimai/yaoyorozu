// src/app.rs

use crate::engine::{evaluator::Evaluator, lexer::Lexer, parser::Parser};
use crate::ui::sidebar;
use crate::ui_theme;
use eframe::egui;

#[derive(serde::Deserialize, serde::Serialize)]
pub struct 開かれた書物 {
    pub 名前: String,
    pub 本文: String,
    pub 所在: Option<std::path::PathBuf>,
}

#[derive(serde::Deserialize, serde::Serialize)]
#[serde(default)]
pub struct YaoyorozuApp {
    開いている書物: Vec<開かれた書物>,
    選択中の札: usize,
    出力結果: String,
    選択中の色: egui::Color32,
}

impl YaoyorozuApp {
    pub fn new(_cc: &eframe::CreationContext<'_>) -> Self {
        // 保存された状態があれば復元し、なければデフォルトを返します
        if let Some(storage) = _cc.storage {
            return eframe::get_value(storage, eframe::APP_KEY).unwrap_or_default();
        }
        Self::default()
    }
}

impl Default for YaoyorozuApp {
    fn default() -> Self {
        Self {
            開いている書物: vec![
                開かれた書物 {
                    名前: "新規ファイル1".to_owned(),
                    本文: "もし 10 ＝ 10 ならば ｛ 表示 100 ＋ 200 ｝".to_owned(),
                    所在: None,
                },
            ],
            選択中の札: 0,
            出力結果: "ここに結果が出ます".to_owned(),
            選択中の色: egui::Color32::WHITE,
        }
    }
}

impl eframe::App for YaoyorozuApp {
    // 状態を保存する魔法（アプリ終了時などに呼ばれます）
    fn save(&mut self, storage: &mut dyn eframe::Storage) {
        eframe::set_value(storage, eframe::APP_KEY, self);
    }

    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        if ctx.fonts(|f| f.families().len() < 3) {
            ui_theme::setup_custom_fonts(ctx);
        }
        // 🌟 この行の頭に「//」をつけて、無効化してください
        // ui_theme::apply_japanese_visuals(ctx);

        // 1. 屋根（上）
        self.屋根_ヘッダー(ctx);

        // 2. 引出（左）
        self.引出_サイドバー(ctx);

        // 3. 縁側（下）
        self.縁側_出力エリア(ctx);

        // 4. 机（中央）：最後にかくことで残りの領域を占有します
        self.机_メインパネル(ctx);
    }
}

impl YaoyorozuApp {
    fn 屋根_ヘッダー(&mut self, ctx: &egui::Context) {
        egui::TopBottomPanel::top("top_panel").show(ctx, |ui| {
            egui::Frame::none()
                .inner_margin(egui::Margin::symmetric(10.0, 8.0))
                .show(ui, |ui| {
                    let response = ui.interact(ui.max_rect(), ui.id(), egui::Sense::click_and_drag());
                    if response.dragged() {
                        ctx.send_viewport_cmd(egui::ViewportCommand::StartDrag);
                    }

                    ui.horizontal(|ui| {
                        ui.heading("🌸");
                        ui.add_space(8.0);
                        ui.separator();

                        ui.menu_button("ファイル", |ui| {
                            if ui.button("📂 開く").clicked() {
                                if let Some(path) = rfd::FileDialog::new()
                                    .add_filter("八百万ファイル", &["yaoyorozu", "txt"])
                                    .pick_file() {
                                    if let Ok(content) = std::fs::read_to_string(&path) {
                                        let name = path.file_name().unwrap().to_string_lossy().into_owned();
                                        self.開いている書物.push(開かれた書物 { 
                                            名前: name, 
                                            本文: content, 
                                            所在: Some(path) 
                                        });
                                        self.選択中の札 = self.開いている書物.len() - 1;
                                    }
                                }
                                ui.close_menu();
                            }
                            if ui.button("💾 保存").clicked() {
                                let current_file = &mut self.開いている書物[self.選択中の札];
                                if current_file.所在.is_none() {
                                    if let Some(path) = rfd::FileDialog::new().save_file() {
                                        current_file.所在 = Some(path);
                                    }
                                }
                                if let Some(path) = &current_file.所在 {
                                    let _ = std::fs::write(path, &current_file.本文);
                                    current_file.名前 = path.file_name().unwrap().to_string_lossy().into_owned();
                                }
                                ui.close_menu();
                            }
                        });

                        ui.separator();

                        egui::ScrollArea::horizontal().id_source("tab_scroll").show(ui, |ui| {
                            ui.horizontal(|ui| {
                                for i in 0..self.開いている書物.len() {
                                    let label = &self.開いている書物[i].名前;
                                    if ui.selectable_label(self.選択中の札 == i, label).clicked() {
                                        self.選択中の札 = i;
                                    }
                                }
                            });
                        });

                        if ui.button("＋").clicked() {
                            self.開いている書物.push(開かれた書物 {
                                名前: format!("新規ファイル{}", self.開いている書物.len() + 1),
                                本文: String::new(),
                                所在: None,
                            });
                            self.選択中の札 = self.開いている書物.len() - 1;
                        }

                        ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                            ui.add_space(3.0);
                            if ui.button("❌").clicked() {
                                ctx.send_viewport_cmd(egui::ViewportCommand::Close);
                            }
                        });
                    });
                });
        });
    }

    fn 引出_サイドバー(&mut self, ctx: &egui::Context) {
        egui::SidePanel::left("side_panel")
            .resizable(true)
            .default_width(150.0)
            .show(ctx, |ui| {
                sidebar::show_file_list(ui, &self.開いている書物, &mut self.選択中の札);
            });
    }

    fn 縁側_出力エリア(&mut self, ctx: &egui::Context) {
        egui::TopBottomPanel::bottom("縁側パネル")
            .resizable(true)
            .default_height(100.0)
            .show(ctx, |ui| {
                ui.horizontal(|ui| {
                    ui.heading("縁側（出力）");
                    if ui.button("▶ 起動").clicked() {
                        self.出力結果 = "八百万のエンジン、起動しました。".to_string();
                    }
                });
                ui.separator();
                egui::ScrollArea::vertical().show(ui, |ui| {
                    ui.label(&self.出力結果);
                });
            });
    }

    fn 机_メインパネル(&mut self, ctx: &egui::Context) {
        egui::CentralPanel::default().show(ctx, |ui| {
            let current_file = &mut self.開いている書物[self.選択中の札];

            ui.vertical(|ui| {
                ui.label(format!("編集中の書物: {}", current_file.名前));
                
                let mut theme = ui_theme::八百万の装束::new();
                theme.set_color(self.選択中の色);

                let mut layouter = |ui: &egui::Ui, string: &str, wrap_width: f32| {
                    let layout_job = theme.layout(ui, string);
                    let mut job = layout_job;
                    job.wrap.max_width = wrap_width;
                    ui.fonts(|f| f.layout_job(job))
                };

                // app.rs 200行目付近：机_メインパネルの中身

                egui::ScrollArea::vertical().show(ui, |ui| {
                    ui.horizontal_top(|ui| {
                        // 1. 行番号エリア（グレーで数字を並べる）
                        let line_count = current_file.本文.lines().count().max(1);
                        let mut line_numbers_str = String::new();
                        for i in 1..=line_count {
                            line_numbers_str.push_str(&format!("{}\n", i));
                        }

                        ui.add(
                            egui::Label::new(
                                egui::RichText::new(line_numbers_str)
                                    .font(egui::FontId::monospace(14.0))
                                    .color(egui::Color32::from_gray(100))
                            )
                        );

                        ui.separator(); // 縦の仕切り線

                        // 2. エディタエリア
                        ui.add_sized(
                            ui.available_size(),
                            egui::TextEdit::multiline(&mut current_file.本文)
                                .font(egui::TextStyle::Monospace)
                                .code_editor()
                                .lock_focus(true)
                                .desired_width(f32::INFINITY)
                                .layouter(&mut layouter),
                        );
                    });
                });
            });
        });
    }
}