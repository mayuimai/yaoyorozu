// src/engine/runner.rs
use crate::engine;

#[derive(serde::Deserialize, serde::Serialize, Default)]
pub struct 起動装置 {
    // 将来ここに「実行状態」などを持たせられます
}

impl 起動装置 {
    pub fn 実行する(&self, ソースコード: &str) -> String {
        // 現在の engine::実行 を呼び出す役割をここにまとめます
        engine::実行(ソースコード)
    }
}