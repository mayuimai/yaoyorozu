use crate::engine::lexer::{Lexer, Token};
use crate::engine::ast::{命令, 式};

pub struct Parser {
    lexer: Lexer,
    current_token: Token,
}

impl Parser {
    pub fn new(mut lexer: Lexer) -> Self {
        let first_token = lexer.次のトークンを出す();
        Self {
            lexer,
            current_token: first_token,
        }
    }

    fn advance(&mut self) {
        self.current_token = self.lexer.次のトークンを出す();
    }

    pub fn 解析する(&mut self) -> Vec<命令> {
        let mut プログラム = Vec::new();
        while self.current_token != Token::終端 {
            if let Some(cmd) = self.命令を解析する() {
                プログラム.push(cmd);
            } else {
                self.advance(); // 解析できない場合は次へ
            }
        }
        プログラム
    }

    fn 命令を解析する(&mut self) -> Option<命令> {
        match self.current_token {
            Token::若し => self.若し文を解析(),
            Token::表示 => {
                self.advance();
                Some(命令::表示文(式::数値(0.0))) // 仮の実装
            }
            _ => None,
        }
    }

    fn 若し文を解析(&mut self) -> Option<命令> {
        self.advance(); // 「若し」を飛ばす

        // 条件の解析（数値 ＝ 数値）
        let 左辺 = if let Token::数値(n) = self.current_token { 式::数値(n) } else { return None; };
        self.advance();
        
        if self.current_token != Token::等号 { return None; }
        self.advance();
        
        let 右辺 = if let Token::数値(n) = self.current_token { 式::数値(n) } else { return None; };
        self.advance();

        // 「ならば」のチェック
        if self.current_token != Token::ならば {
            return None;
        }
        self.advance();

        // 「｛」のチェック
        if self.current_token != Token::左中括弧 {
            return None;
        }
        self.advance();

        // ブロック内の中身を解析
        let mut 実行内容 = Vec::new();
        while self.current_token != Token::右中括弧 && self.current_token != Token::終端 {
            if let Some(cmd) = self.命令を解析する() {
                実行内容.push(cmd);
            }
            self.advance();
        }

        Some(命令::若し文 {
            条件: 式::比較 {
                左辺: Box::new(左辺),
                右辺: Box::new(右辺),
            },
            実行内容,
        })
    }
}