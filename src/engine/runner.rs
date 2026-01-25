// src/engine/runner.rs

#[derive(serde::Deserialize, serde::Serialize, Default)]
pub struct 起動装置 {
    // 状態を持たせるための定義
}

impl 起動装置 {
    pub fn 実行する(&self, ソースコード: &str) -> String {
        // 親モジュール(mod.rs)の実行関数を呼び出す
        super::実行(ソースコード)
    }

    // ファイルを直接読み込んで実行する便利な機能
    pub fn ファイルを実行する(&self, パス: &str) -> String {
        match std::fs::read_to_string(パス) {
            Ok(コード) => self.実行する(&コード),
            Err(_) => format!("❌ ファイル「{}」が見つかりませんでした。", パス),
        }
    }
}