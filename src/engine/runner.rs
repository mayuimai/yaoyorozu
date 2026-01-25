// src/engine/runner.rs

// ❌ use crate::engine; は消してOKです

#[derive(serde::Deserialize, serde::Serialize, Default)]
pub struct 起動装置 {
    // 将来ここに「実行状態」などを持たせられます
}

impl 起動装置 {
    pub fn 実行する(&self, ソースコード: &str) -> String {
        // 🌟 修正ポイント：親（mod.rs）の関数を呼びます
        super::実行(ソースコード)
    }
}