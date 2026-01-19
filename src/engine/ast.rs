// ast.rs 

#[derive(Debug, Clone)]
pub enum 命令 {
    もし文 {
        条件: 式,
        実行内容: Vec<命令>,
    },
    表示文(式),
    記録文, // ← これ！
    送信文, // ← これ！
}

#[derive(Debug, Clone)]
pub enum 式 {
    数値(f64),
    // 比較（＝, ＞, ＜ など）
    比較 {
        左辺: Box<式>,
        演算子: String,
        右辺: Box<式>,
    },
    // 計算（＋, －, ＊, ／ など）
    計算 {
        左辺: Box<式>,
        演算子: char,
        右辺: Box<式>,
    },
}