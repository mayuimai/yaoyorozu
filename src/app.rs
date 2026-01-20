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
                    本文: "※ ここに言霊を記してください\nもし 10 ＝ 10 ならば ｛ 表示 100 ＋ 200 ｝".to_owned(),
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
    fn save(&mut self, storage: &mut dyn eframe::Storage) {
        eframe::set_value(storage, eframe::APP_KEY, self);
    }

    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        if ctx.fonts(|f| f.families().len() < 3) {
            ui_theme::setup_custom_fonts(ctx);
        }

        self.屋根_ヘッダー(ctx);
        self.引出_サイドバー(ctx);
        self.縁側_出力エリア(ctx);
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
                        //ui.separator();

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

                        //ui.separator();

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
        egui::TopBottomPanel::bottom("output_panel")
            .resizable(true)
            .default_height(150.0)
            .show(ctx, |ui| {
                ui.horizontal(|ui| {
                    ui.heading("縁側（出力エリア）");
                    if ui.button("▶ 起動").clicked() {
                        let ソースコード = &self.開いている書物[self.選択中の札].本文;
                        self.出力結果 = crate::engine::実行(ソースコード);
                    }
                    if ui.button("🗑 掃除").clicked() {
                        self.出力結果.clear();
                    }
                });
                
                egui::ScrollArea::vertical().show(ui, |ui| {
                    ui.add(
                        egui::Label::new(
                            egui::RichText::new(&self.出力結果)
                                .font(egui::FontId::monospace(14.0))
                        ).wrap()
                    );
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

                egui::ScrollArea::vertical().show(ui, |ui| {
                    // 🌟 エディタの margin と合わせる
                    ui.add_space(11.0);

                    ui.horizontal_top(|ui| {
                        // 1. 行番号エリア
                        ui.add_space(10.0);
                        
                        let line_count = current_file.本文.lines().count().max(1);
                        let mut job = egui::text::LayoutJob::default();
                        for i in 1..=line_count {
                            job.append(
                                &format!("{}\n", i),
                                0.0,
                                egui::TextFormat {
                                    font_id: egui::FontId::monospace(14.0),
                                    color: egui::Color32::from_gray(100),
                                    line_height: Some(21.0), // 🌟 21.0 で固定
                                    ..Default::default()
                                },
                            );
                        }
                        // --- ここから書き換え ---
                        ui.allocate_ui(egui::vec2(30.0, 0.0), |ui| {
                            egui::Frame::none()
                                .inner_margin(egui::Margin {
                                    top: 10.0,    // 🌟 ここで行番号だけ「2ピクセル」下に下げます
                                    left: 0.0,
                                    right: 0.0,
                                    bottom: 0.0,
                                })
                                .show(ui, |ui| {
                                    ui.add(egui::Label::new(job).wrap());
                                });
                        });
                        // --- ここまで ---

                        ui.add_space(8.0);
                        //ui.separator();

                        // 2. エディタエリア
                        ui.add_sized(
                            ui.available_size(),
                            egui::TextEdit::multiline(&mut current_file.本文)
                                .font(egui::TextStyle::Monospace)
                                .code_editor()
                                .lock_focus(true)
                                .margin(egui::vec2(10.0, 10.0))
                                .desired_width(f32::INFINITY)
                                .layouter(&mut layouter),
                        );
                    });
                });
            });
        });
    }
}