//evaluator.rs

use crate::engine::ast::{命令, 式};
use std::cell::RefCell;

pub struct Evaluator {
    出力バッファ: RefCell<String>,
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
            命令::もし文 {
                条件, 実行内容
            } => {
                if self.論理評価(条件) {
                    for 内側の命令 in 実行内容 {
                        self.命令を実行(内側の命令);
                    }
                }
            }
            命令::表示文(内容) => {
                let 値 = self.数値評価(内容);
                let mut buffer = self.出力バッファ.borrow_mut();
                buffer.push_str(&format!("【出力】: {}\n", 値));
            }
            // --- Git操作の魔法 ---
            命令::記録文 => {
                let mut buffer = self.出力バッファ.borrow_mut();
                buffer.push_str("【記録中】: 変更を保存しています...\n");
                let _ = std::process::Command::new("git")
                    .args(["add", "."])
                    .output();
                let _ = std::process::Command::new("git")
                    .args(["commit", "-m", "八百万エディタからの自動記録"])
                    .output();
                buffer.push_str("【完了】: 記録されました。🌸\n");
            }
            命令::送信文 => {
                let mut buffer = self.出力バッファ.borrow_mut();
                buffer.push_str("【送信中】: GitHubへ送り届けています...\n");
                let output = std::process::Command::new("git")
                    .args(["push", "origin", "main"])
                    .output();
                match output {
                    Ok(_) => buffer.push_str("【完了】: GitHubへ無事に届きました！🚀\n"),
                    Err(e) => buffer.push_str(&format!("【失敗】: 送信できませんでした: {}\n", e)),
                }
            }
        } // ← この match の閉じカッコが重要です！
    }

    fn 論理評価(&self, expr: 式) -> bool {
        match expr {
            式::比較 {
                左辺, 演算子, 右辺
            } => {
                let 左 = self.数値評価(*左辺);
                let 右 = self.数値評価(*右辺);
                if 演算子 == "＝" {
                    左 == 右
                } else {
                    false
                }
            }
            _ => self.数値評価(expr) != 0.0,
        }
    }

    fn 数値評価(&self, expr: 式) -> f64 {
        match expr {
            式::数値(n) => n,
            式::計算 {
                左辺, 演算子, 右辺
            } => {
                let 左 = self.数値評価(*左辺);
                let 右 = self.数値評価(*右辺);
                match 演算子 {
                    '+' => 左 + 右,
                    '-' => 左 - 右,
                    '*' => 左 * 右,
                    '/' => {
                        if 右 != 0.0 {
                            左 / 右
                        } else {
                            0.0
                        }
                    }
                    _ => 0.0,
                }
            }
            式::比較 { .. } => {
                if self.論理評価(expr) {
                    1.0
                } else {
                    0.0
                }
            }
        }
    }
}
