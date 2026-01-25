use eframe::egui;
use crate::ui_theme;
use crate::ui::sidebar; // サイドバーを使う
use crate::engine::runner::起動装置;

// 🌟 これが sidebar.rs で必要だった「開かれた書物」です！
#[derive(Clone)]
pub struct 開かれた書物 {
    pub 名前: String,
    pub 中身: String,
    pub 保存済み: bool,
}

pub struct YaoyorozuApp {
    // 複数のファイルを持てるように進化させました
    files: Vec<開かれた書物>,
    active_tab: usize, // 今どのファイルを見ているか
    
    output_log: String,
    sidebar_expanded: bool,
    engine: 起動装置,
    theme: ui_theme::八百万の装束,
}

impl Default for YaoyorozuApp {
    fn default() -> Self {
        // 初期ファイルとして runner.8g を用意
        let runner_code = std::fs::read_to_string("src/engine/runner.8g")
            .unwrap_or_else(|_| "名前 ＝ 「繭」\n表示 名前".to_string());

        Self {
            files: vec![
                開かれた書物 { 名前: "runner.8g".to_string(), 中身: runner_code, 保存済み: true },
                開かれた書物 { 名前: "memo.txt".to_string(), 中身: "メモ帳として使えます".to_string(), 保存済み: true },
            ],
            active_tab: 0,
            output_log: "ここに実行結果が表示されます...".to_owned(),
            sidebar_expanded: true,
            engine: 起動装置::default(),
            theme: ui_theme::八百万の装束::new(),
        }
    }
}

impl YaoyorozuApp {
    pub fn new(cc: &eframe::CreationContext<'_>) -> Self {
        ui_theme::setup_custom_fonts(&cc.egui_ctx);
        ui_theme::apply_japanese_visuals(&cc.egui_ctx);
        Self::default()
    }
}

impl eframe::App for YaoyorozuApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        
        // 1. 左サイドバー（Arc風）
        if self.sidebar_expanded {
            egui::SidePanel::left("sidebar_panel")
                .resizable(true)
                .default_width(200.0)
                .show(ctx, |ui| {
                    // 🌟 ここで sidebar.rs の関数を呼び出します
                    sidebar::render(ui, &mut self.files, &mut self.active_tab);
                });
        }

        // 2. 下部パネル（ターミナル）
        egui::TopBottomPanel::bottom("bottom_panel")
            .resizable(true)
            .default_height(150.0)
            .show(ctx, |ui| {
                ui.horizontal(|ui| {
                    ui.label("📺 出力");
                    if ui.button("クリア").clicked() { self.output_log.clear(); }
                });
                ui.separator();
                egui::ScrollArea::vertical().stick_to_bottom(true).show(ui, |ui| {
                    ui.monospace(&self.output_log);
                });
            });

        // 3. 中央パネル（エディタ）
        egui::CentralPanel::default().show(ctx, |ui| {
            // ヘッダー（ファイル名と実行ボタン）
            ui.horizontal(|ui| {
                if ui.button(if self.sidebar_expanded { "◀" } else { "▶" }).clicked() {
                    self.sidebar_expanded = !self.sidebar_expanded;
                }
                
                // 現在開いているファイル名を表示
                if let Some(active_file) = self.files.get(self.active_tab) {
                    ui.label(egui::RichText::new(format!(" {} ", active_file.名前)).strong().background_color(egui::Color32::from_gray(230)));
                }

                ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                    if ui.button("▶ 実行").clicked() {
                        if let Some(file) = self.files.get(self.active_tab) {
                            if file.名前 == "runner.8g" {
                                let _ = std::fs::write("src/engine/runner.8g", &file.中身);
                                self.output_log = self.engine.実行する(&file.中身);
                            } else {
                                self.output_log = "※ .8g ファイル以外は実行できません".to_string();
                            }
                        }
                    }
                    if ui.button("💾 保存").clicked() {
                         if let Some(file) = self.files.get(self.active_tab) {
                             if file.名前 == "runner.8g" {
                                 let _ = std::fs::write("src/engine/runner.8g", &file.中身);
                             }
                             self.output_log.push_str("\n【保存】: 保存しました。");
                         }
                    }
                });
            });

            ui.separator();

            // エディタ本体
            if let Some(file) = self.files.get_mut(self.active_tab) {
                egui::ScrollArea::vertical().show(ui, |ui| {
                    let theme = &self.theme;
                    egui::TextEdit::multiline(&mut file.中身)
                        .font(egui::TextStyle::Monospace)
                        .desired_width(f32::INFINITY)
                        .desired_rows(20)
                        .lock_focus(true)
                        .layouter(&mut |ui, string, wrap_width| {
                            theme.layout(ui, string, wrap_width)
                        })
                        .show(ui);
                });
            }
        });
    }
}