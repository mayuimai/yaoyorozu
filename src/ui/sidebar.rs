use eframe::egui;
use crate::app::開かれた書物;

// サイドバーを描画する関数
pub fn render(ui: &mut egui::Ui, files: &mut Vec<開かれた書物>, active_tab: &mut usize) {
    ui.add_space(10.0);
    ui.heading("📂 八百万");
    ui.separator();

    ui.label("ワークスペース");
    ui.add_space(5.0);

    // ファイル一覧をボタンとして表示（Arc風の垂直タブ）
    for (index, file) in files.iter().enumerate() {
        let is_selected = *active_tab == index;
        
        // 選択されているタブは色を変える
        let button = egui::Button::new(if is_selected {
            format!("Running  {}", file.名前) 
        } else {
            format!("📄 {}", file.名前)
        }).frame(false); // 枠線を消してモダンに

        if ui.add(button).clicked() {
            *active_tab = index;
        }
    }

    ui.with_layout(egui::Layout::bottom_up(egui::Align::Min), |ui| {
        ui.label("v0.1.0");
        ui.separator();
        ui.label("🧩 拡張機能");
    });
}