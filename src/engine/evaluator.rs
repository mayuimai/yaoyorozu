use crate::engine::ast::{命令, 式};

pub struct Evaluator;

impl Evaluator {
    pub fn 実行(&self, 命令セット: Vec<命令>) {
        for cmd in 命令セット {
            self.命令を実行(cmd);
        }
    }

    fn 命令を実行(&self, cmd: 命令) {
        match cmd {
            命令::若し文 { 条件, 実行内容 } => {
                if self.式を評価(条件) {
                    for 内側の命令 in 実行内容 {
                        self.命令を実行(内側の命令);
                    }
                }
            }
            命令::表示文(内容) => {
                // ここで実際に画面に文字を出す！
                println!("【出力】: {}", self.数値として評価(内容));
            }
        }
    }

    fn 式を評価(&self, expr: 式) -> bool {
        match expr {
            式::比較 { 左辺, 右辺 } => {
                self.数値として評価(*左辺) == self.数値として評価(*右辺)
            }
            _ => false,
        }
    }

    fn 数値として評価(&self, expr: 式) -> f64 {
        match expr {
            式::数値(n) => n,
            _ => 0.0,
        }
    }
}