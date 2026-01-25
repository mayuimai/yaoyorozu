// 1行目：engineフォルダがあることをRustに教えます
pub mod engine;

use std::fs;
use crate::engine::runner::起動装置;

fn main() {
    let 装置 = engine::runner::起動装置::default();
    // パスを画像の位置（src/engine/）に合わせます
    let 結果 = 装置.ファイルを実行する("src/engine/runner.8g");
    println!("{}", 結果);
}