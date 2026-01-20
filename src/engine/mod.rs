// src/engine/mod.rs
pub mod ast;
pub mod evaluator;
pub mod lexer;
pub mod parser;
pub mod runner;

// 窓口（公開関数）を一つ作っておく
pub fn 実行(ソースコード: &str) -> String {
    let レキシカ = lexer::Lexer::new(ソースコード);
    let mut パーサ = parser::Parser::new(レキシカ);
    let 構文木 = パーサ.解析する();

    let 実行機 = evaluator::Evaluator::new();
    let 結果 = 実行機.実行(構文木);

    if 結果.is_empty() {
        "（結果なし）".to_owned()
    } else {
        結果
    }
}
