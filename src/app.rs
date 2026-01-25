use eframe::egui;
use crate::ui_theme;
use crate::engine::runner::起動装置;

pub struct YaoyorozuApp {
    code: String,
    output_log: String,
    sidebar_expanded: bool,
    engine: 起動装置,
    theme: ui_theme::八百万の装束, // テーマを保持
}

impl Default for YaoyorozuApp {
    fn default() -> Self {
        Self {
            // 起動時に runner.8g を読み込む（なければデフォルト文）
            code: std::fs::read_to_string("src/engine/runner.8g")
                  .unwrap_or_else(|_| "表示 「ようこそ」".to_string()),
            output_log: "ここに実行結果が表示されます...".to_owned(),
            sidebar_expanded: true,
            engine: 起動装置::default(),
            theme: ui_theme::八百万の装束::new(),
        }
    }
}

impl YaoyorozuApp {
    pub fn new(cc: &eframe::CreationContext<'_>) -> Self {
        // フォントとテーマの適用
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
                    ui.add_space(10.0);
                    ui.heading("📂 八百万");
                    ui.separator();
                    
                    if ui.button("📄 runner.8g").clicked() {
                        // ファイル選択（今は表示だけ）
                    }
                    ui.label("📄 memo.txt");

                    ui.with_layout(egui::Layout::bottom_up(egui::Align::Min), |ui| {
                        ui.label("v0.1.0");
                        ui.separator();
                        ui.label("🧩 拡張機能");
                    });
                });
        }

        // 2. 下部パネル（ターミナル）
        egui::TopBottomPanel::bottom("bottom_panel")
            .resizable(true)
            .default_height(150.0)
            .show(ctx, |ui| {
                ui.horizontal(|ui| {
                    ui.label("📺 出力");
                    if ui.button("クリア").clicked() {
                        self.output_log.clear();
                    }
                });
                ui.separator();
                egui::ScrollArea::vertical().stick_to_bottom(true).show(ui, |ui| {
                    ui.monospace(&self.output_log);
                });
            });

        // 3. 中央パネル（エディタ）
        egui::CentralPanel::default().show(ctx, |ui| {
            // ヘッダーバー
            ui.horizontal(|ui| {
                if ui.button(if self.sidebar_expanded { "◀" } else { "▶" }).clicked() {
                    self.sidebar_expanded = !self.sidebar_expanded;
                }
                ui.label(" runner.8g ");
                
                ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                    // 実行ボタン
                    if ui.button("▶ 実行").clicked() {
                        // エディタの内容を保存して実行！
                        let _ = std::fs::write("src/engine/runner.8g", &self.code);
                        self.output_log = self.engine.実行する(&self.code);
                    }
                    if ui.button("💾 保存").clicked() {
                        let _ = std::fs::write("src/engine/runner.8g", &self.code);
                        self.output_log.push_str("\n【保存】: ファイルを保存しました。");
                    }
                });
            });

            ui.separator();

            // エディタエリア（シンタックスハイライト付き！）
            egui::ScrollArea::vertical().show(ui, |ui| {
                let theme = &self.theme;
                egui::TextEdit::multiline(&mut self.code)
                    .font(egui::TextStyle::Monospace)
                    .desired_width(f32::INFINITY)
                    .desired_rows(20)
                    .lock_focus(true)
                    .layouter(&mut |ui, string, wrap_width| {
                        theme.layout(ui, string, wrap_width)
                    })
                    .show(ui);
            });
        });
    }
}