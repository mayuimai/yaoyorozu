use crate::engine::ast::{命令, 式};
use std::cell::RefCell;

pub struct Evaluator {
    出力バッファ: RefCell<String>,
}

#[derive(Debug, Clone, PartialEq)]
pub enum 値 {
    数値(f64),
    文字列(String),
    空,
}

impl Evaluator {
    pub fn new() -> Self {
        Self {
            出力バッファ: RefCell::new(String::new()),
        }
    }

    pub fn 実行(&self, 命令セット: Vec<命令>) -> String {
        for cmd in 命令セット {
            self.命令を実行(cmd);
        }
        self.出力バッファ.borrow().clone()
    }

    fn 命令を実行(&self, cmd: 命令) {
        match cmd {
            命令::もし文 { 条件, 実行内容 } => {
                if self.論理評価(条件) {
                    for 内側の命令 in 実行内容 {
                        self.命令を実行(内側の命令);
                    }
                }
            }
            命令::表示文(内容) => {
                let 評価結果 = self.評価(内容);
                let mut buffer = self.出力バッファ.borrow_mut();
                
                // 値の種類によって表示を切り替える
                let 表示テキスト = match 評価結果 {
                    値::数値(n) => n.to_string(),
                    値::文字列(s) => s,
                    値::空 => "（空）".to_string(),
                };
                buffer.push_str(&format!("【出力】: {}\n", 表示テキスト));
            }
            // --- Git操作 ---
            命令::記録文 => {
                let mut buffer = self.出力バッファ.borrow_mut();
                buffer.push_str("【記録中】: 変更を保存しています...\n");
                let _ = std::process::Command::new("git").args(["add", "."]).output();
                let _ = std::process::Command::new("git").args(["commit", "-m", "八百万エディタからの自動記録"]).output();
                buffer.push_str("【完了】: 記録されました。🌸\n");
            }
            命令::送信文 => {
                let mut buffer = self.出力バッファ.borrow_mut();
                buffer.push_str("【送信中】: GitHubへ送り届けています...\n");
                let output = std::process::Command::new("git").args(["push", "origin", "main"]).output();
                match output {
                    Ok(_) => buffer.push_str("【完了】: GitHubへ無事に届きました！🚀\n"),
                    Err(e) => buffer.push_str(&format!("【失敗】: 送信できませんでした: {}\n", e)),
                }
            }
        }
    }

    fn 論理評価(&self, expr: 式) -> bool {
        match expr {
            式::比較 { 左辺, 演算子, 右辺 } => {
                let 左 = self.評価(*左辺);
                let 右 = self.評価(*右辺);
                if 演算子 == "＝" {
                    左 == 右
                } else {
                    false
                }
            }
            _ => match self.評価(expr) {
                値::数値(n) => n != 0.0,
                値::文字列(s) => !s.is_empty(),
                値::空 => false,
            },
        }
    }

    // 万能な評価関数
    fn 評価(&self, expr: 式) -> 値 {
        match expr {
            式::数値(n) => 値::数値(n),
            式::文字列(s) => 値::文字列(s),
            式::計算 { 左辺, 演算子, 右辺 } => {
                // 計算のときは数値として取り出す
                let 左 = match self.評価(*左辺) { 値::数値(n) => n, _ => 0.0 };
                let 右 = match self.評価(*右辺) { 値::数値(n) => n, _ => 0.0 };
                let 結果 = match 演算子 {
                    '+' => 左 + 右,
                    '-' => 左 - 右,
                    '*' => 左 * 右,
                    '/' => if 右 != 0.0 { 左 / 右 } else { 0.0 },
                    _ => 0.0,
                };
                値::数値(結果)
            }
            式::比較 { .. } => {
                if self.論理評価(expr) { 値::数値(1.0) } else { 値::数値(0.0) }
            }
        }
    }
}