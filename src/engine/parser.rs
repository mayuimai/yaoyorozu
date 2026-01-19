use crate::engine::lexer::{Lexer, Token};
use crate::engine::ast::{命令, 式};

pub struct Parser {
    lexer: Lexer,
    current_token: Token,
}

impl Parser {
    pub fn new(mut lexer: Lexer) -> Self {
        // ここで E0599 が出ている場合、lexer.rs の保存を確認してください！
        let first_token = lexer.次のトークンを出す();
        Self {
            lexer,
            current_token: first_token,
        }
    }

    fn advance(&mut self) {
        // ここ！ Lexer側で「pub fn 次のトークンを出す」となっている必要があります
        self.current_token = self.lexer.次のトークンを出す();
    }

    pub fn 解析する(&mut self) -> Vec<命令> {
        let mut プログラム = Vec::new();
        while self.current_token != Token::終端 {
            if let Some(cmd) = self.命令を解析する() {
                プログラム.push(cmd);
            } else {
                self.advance();
            }
        }
        プログラム
    }

    fn 命令を解析する(&mut self) -> Option<命令> {
        match self.current_token {
            Token::もし => self.もし文を解析(),
            Token::表示 => self.表示文を解析(),
            _ => None,
        }
    }

    fn 表示文を解析(&mut self) -> Option<命令> {
        self.advance();
        let 内容 = self.式を解析する()?;
        Some(命令::表示文(内容))
    }

    fn もし文を解析(&mut self) -> Option<命令> {
        self.advance(); // 「もし」を飛ばす
        let 条件 = self.式を解析する()?;

        if self.current_token != Token::ならば { return None; }
        self.advance(); // 「ならば」を飛ばす

        let mut 実行内容 = Vec::new();
        
        // 「終わり」が来るまで命令を読み込み続けるように変更
        while self.current_token != Token::終わり && self.current_token != Token::終端 {
            if let Some(cmd) = self.命令を解析する() {
                実行内容.push(cmd);
            } else {
                self.advance();
            }
        }
        
        if self.current_token == Token::終わり {
            self.advance(); // 「終わり」を飛ばす
        }

        Some(命令::もし文 { 条件, 実行内容 })
    }

    // 優先順位： 1:比較(＝) , 2:加減(＋, －) , 3:乗除(＊, ／)

    fn 式を解析する(&mut self) -> Option<式> {
        self.比較の解析()
    }

    fn 比較の解析(&mut self) -> Option<式> {
        let mut 左辺 = self.加減の解析()?;
        while self.current_token == Token::等号 {
            self.advance();
            let 右辺 = self.加減の解析()?;
            左辺 = 式::比較 {
                左辺: Box::new(左辺),
                演算子: "＝".to_string(),
                右辺: Box::new(右辺),
            };
        }
        Some(左辺)
    }

    fn 加減の解析(&mut self) -> Option<式> {
        let mut 左辺 = self.乗除の解析()?;
        while self.current_token == Token::加算 || self.current_token == Token::減算 {
            let op = if self.current_token == Token::加算 { '+' } else { '-' };
            self.advance();
            let 右辺 = self.乗除の解析()?;
            左辺 = 式::計算 {
                左辺: Box::new(左辺),
                演算子: op,
                右辺: Box::new(右辺),
            };
        }
        Some(左辺)
    }

    fn 乗除の解析(&mut self) -> Option<式> {
        // 1. まずは「最初の数」か「カッコの中身」を確定させる
        let mut 左辺 = match self.current_token {
            Token::数値(n) => {
                self.advance(); // 数値を読んだので次へ
                式::数値(n)
            }
            Token::左括弧 => {
                self.advance(); // 「（」を読んだので次へ
                let 内部の式 = self.式を解析する()?;
                if self.current_token == Token::右括弧 {
                    self.advance(); // 「）」を読んだので次へ
                }
                内部の式
            }
            _ => return None,
        };

        // 2. その後に「＊」や「／」が続く限り、計算を繋げていく
        while self.current_token == Token::乗算 || self.current_token == Token::除算 {
            let op = if self.current_token == Token::乗算 { '*' } else { '/' };
            self.advance(); // 演算子を読んだので次へ
            
            let 右辺 = match self.current_token {
                Token::数値(n) => {
                    self.advance();
                    式::数値(n)
                }
                Token::左括弧 => {
                    self.advance();
                    let 内部 = self.式を解析する()?;
                    if self.current_token == Token::右括弧 { self.advance(); }
                    内部
                }
                _ => return None,
            };

            左辺 = 式::計算 {
                左辺: Box::new(左辺),
                演算子: op,
                右辺: Box::new(右辺),
            };
        }
        Some(左辺)
    }
}