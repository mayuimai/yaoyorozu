// src/engine/mod.rs

pub mod ast;
pub mod evaluator;
pub mod lexer;
pub mod parser;
pub mod runner;

use lexer::Lexer;
use parser::Parser;
use evaluator::Evaluator;

// 🚀 本格始動：うさぎ組の知能をすべてつなぎます
pub fn 実行(ソースコード: &str) -> String {
    // 1. 耳（Lexer）：言葉を単語に分ける
    let レキシカ = Lexer::new(ソースコード);

    // 2. 脳（Parser）：単語を並べて「意味（構文木）」を作る
    let mut パーサ = Parser::new(レキシカ);
    let 構文木 = パーサ.解析する();

    // 3. 手足（Evaluator）：命令を実際に動かす
    let 実行機 = Evaluator::new();
    let 結果 = 実行機.実行(構文木);

    if 結果.is_empty() {
        "（命令は終わったけど、何も表示されなかったよ）".to_owned()
    } else {
        format!("🐰 うさぎ組の実行結果：\n{}", 結果)
    }
}