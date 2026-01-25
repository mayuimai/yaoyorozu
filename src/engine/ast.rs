// src/engine/ast.rs

#[derive(Debug, Clone)]
pub enum 命令 {
    変数宣言 { 名前: String, 値: 式 },
    代入文 { 名前: String, 値: 式 },   // 🌟 Pythonスタイル用
    もし文 { 条件: 式, 実行内容: Vec<命令>, さもなくば: Vec<命令> },
    表示文(式),
    記録文,
    送信文,
}

#[derive(Debug, Clone)]
pub enum 式 {
    数値(f64),
    文字列(String),
    変数(String), // 変数の中身を読み出す
    比較 { 左辺: Box<式>, 演算子: String, 右辺: Box<式> },
    計算 { 左辺: Box<式>, 演算子: char, 右辺: Box<式> },
}