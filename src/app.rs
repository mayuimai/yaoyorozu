//app.rs


use eframe::egui;
use crate::engine::{lexer::Lexer, parser::Parser, evaluator::Evaluator};
use crate::ui_theme;
use crate::ui::sidebar; // サイドバーモジュールを呼び出し

// カラーコード（例: "#FFFFFF"）を読み取る魔法の道具
fn hex(hex_str: &str) -> egui::Color32 {
    let r = u8::from_str_radix(&hex_str[1..3], 16).unwrap_or(0);
    let g = u8::from_str_radix(&hex_str[3..5], 16).unwrap_or(0);
    let b = u8::from_str_radix(&hex_str[5..7], 16).unwrap_or(0);
    egui::Color32::from_rgb(r, g, b)
}

pub struct OpenedFile {
    pub name: String,
    pub content: String,
    pub path: Option<std::path::PathBuf>,
}

pub struct YaoyorozuApp {
    files: Vec<OpenedFile>,
    active_tab: usize,
    出力結果: String,
    // --- 追加：選んだ色を覚えておく場所 ---
    選択中の色: egui::Color32,
}

impl YaoyorozuApp {
    pub fn new(_cc: &eframe::CreationContext<'_>) -> Self {
        Self::default()
    }
}

impl Default for YaoyorozuApp {
    fn default() -> Self {
        Self {
            files: vec![
                OpenedFile {
                    name: "新規ファイル1".to_owned(),
                    content: "もし 10 ＝ 10 ならば ｛ 表示 100 ＋ 200 ｝".to_owned(),
                    path: None,
                },
            ],
            active_tab: 0,
            出力結果: "ここに結果が出ます".to_owned(),
            選択中の色: egui::Color32::WHITE,
        }
    }
}

impl eframe::App for YaoyorozuApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        if ctx.fonts(|f| f.families().len() < 3) {
            ui_theme::setup_custom_fonts(ctx);
        }
        ui_theme::apply_japanese_visuals(ctx);

        // --- 1. 自作タイトルバー兼ヘッダー ---
        egui::TopBottomPanel::top("top_panel").show(ctx, |ui| {
            // Paddingを追加：上下4px、左右8pxの余白
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
                                            self.files.push(OpenedFile { name, content, path: Some(path) });
                                            self.active_tab = self.files.len() - 1;
                                        }
                                    }
                                ui.close_menu();
                            }
                            if ui.button("💾 保存").clicked() {
                                let current_file = &mut self.files[self.active_tab];
                                if current_file.path.is_none() {
                                    if let Some(path) = rfd::FileDialog::new().save_file() {
                                        current_file.path = Some(path);
                                    }
                                }
                                if let Some(path) = &current_file.path {
                                    let _ = std::fs::write(path, &current_file.content);
                                    current_file.name = path.file_name().unwrap().to_string_lossy().into_owned();
                                }
                                ui.close_menu();
                            }
                        });

                        ui.separator();

                        egui::ScrollArea::horizontal().id_source("tab_scroll").show(ui, |ui| {
                            ui.horizontal(|ui| {
                                for i in 0..self.files.len() {
                                    let label = &self.files[i].name;
                                    if ui.selectable_label(self.active_tab == i, label).clicked() {
                                        self.active_tab = i;
                                    }
                                }
                            });
                        });

                        if ui.button("＋").clicked() {
                            self.files.push(OpenedFile {
                                name: format!("新規ファイル{}", self.files.len() + 1),
                                content: String::new(),
                                path: None,
                            });
                            self.active_tab = self.files.len() - 1;
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

        // --- 2. サイドバー (ファイル一覧) ---
        egui::SidePanel::left("side_panel")
            .resizable(true)
            .default_width(150.0)
            .show(ctx, |ui| {
                sidebar::show_file_list(ui, &self.files, &mut self.active_tab);
            });

        // --- 3. メイン編集エリア ---
        egui::CentralPanel::default().show(ctx, |ui| {
            let current_file = &mut self.files[self.active_tab];
            
            egui::ScrollArea::vertical()
                .id_source("editor_scroll")
                .max_height(ui.available_height() - 150.0)
                .show(ui, |ui| {
                ui.horizontal_top(|ui| {
                    let line_count = current_file.content.lines().count().max(1);
                    let mut line_numbers = String::new();
                    for i in 1..=line_count {
                        line_numbers.push_str(&format!("{:>3}\n", i));
                    }
                    ui.add(egui::Label::new(
                        egui::RichText::new(line_numbers)
                            .font(egui::FontId::monospace(14.0))
                            .color(egui::Color32::from_gray(120))
                    ));
                    ui.separator();
                    // --- 修正後のエディタ部分 ---
                    let mut layouter = |ui: &egui::Ui, string: &str, wrap_width: f32| {
                        let mut layout_job = crate::ui::syntax::highlight_yaoyorozu(ui, string);
                        layout_job.wrap.max_width = wrap_width;
                        ui.fonts(|f| f.layout_job(layout_job))
                    };
                    // --- エディタの背景を暗くする枠を追加 ---
                    egui::Frame::none()
                        .fill(egui::Color32::from_rgb(22, 26, 26)) // 羊羹色
                        .inner_margin(egui::Margin::same(10.0))    // 文字が端にくっつかないよう余白
                        .show(ui, |ui| {
                            ui.add(egui::TextEdit::multiline(&mut current_file.content)
                                .desired_rows(20)
                                .font(egui::FontId::monospace(14.0))
                                .desired_width(f32::INFINITY)
                                .min_size(ui.available_size()) // ← これを追加！画面の余っているスペース全部を黒くします
                                .frame(false) // TextEdit自体の枠は消す
                                .layouter(&mut layouter)
                            );
                        });

                    
                });
            });

            ui.add_space(10.0);
            ui.separator();
            ui.add_space(5.0);

            // ⚡ 実行エリア
            ui.horizontal(|ui| {
                ui.visuals_mut().widgets.hovered.bg_fill = egui::Color32::from_rgb(180, 80, 100);
                if ui.add(egui::Button::new(egui::RichText::new("⚡ 実行する").strong())).clicked() {
                    let レキシカ = Lexer::new(&current_file.content);
                    let mut パーサ = Parser::new(レキシカ);
                    let 構文木 = パーサ.解析する();
                    let 実行機 = Evaluator::new();
                    self.出力結果 = 実行機.実行(構文木);
                }
                ui.label(egui::RichText::new("出力結果:").color(egui::Color32::from_gray(180)));

                ui.separator();
                
                // --- ここ！一つの horizontal（横並び）の中にまとめます ---
                ui.label("文字色:");
                ui.color_edit_button_srgba(&mut self.選択中の色);
            }); // ← ここで横並び終了

            ui.add_space(5.0);

            // 📋 結果表示エリアを少し暗くして区別する
            egui::Frame::none()
                .fill(egui::Color32::from_gray(20)) // 少しだけ明るい灰色
                .inner_margin(egui::Margin::same(10.0))
                .rounding(4.0) // 角を少し丸く
                .show(ui, |ui| {
                    egui::ScrollArea::vertical()
                        .id_source("output_scroll")
                        .max_height(ui.available_height() - 150.0)
                        .show(ui, |ui| {
                            // self.選択中の色 を反映させる！
                            ui.add(egui::Label::new(
                                egui::RichText::new(&self.出力結果)
                                    .color(self.選択中の色)
                                    .size(16.0)
                            ));
                        });
                });
        });
    }
}