
use eframe::egui;

// サイドバーの中身を描画する関数
pub fn show_file_list(ui: &mut egui::Ui, files: &[crate::app::開かれた書物], active_tab: &mut usize) {
    ui.add_space(4.0);
    ui.heading("ファイル一覧");
    ui.separator();

    egui::ScrollArea::vertical().show(ui, |ui| {
        ui.vertical(|ui| {
            for i in 0..files.len() {
                let label = &files[i].名前;
                // 選択中のファイルは強調表示
                if ui
                    .selectable_label(*active_tab == i, format!("📄 {}", label))
                    .clicked()
                {
                    *active_tab = i;
                }
            }
        });
    });
}
