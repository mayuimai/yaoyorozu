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
                self.advance();
            }
        }
        プログラム
    }

    fn 命令を解析する(&mut self) -> Option<命令> {
        match self.current_token {
            Token::若し => self.若し文を解析(),
            Token::表示 => self.表示文を解析(),
            _ => None,
        }
    }

    fn 表示文を解析(&mut self) -> Option<命令> {
        self.advance(); // 「表示」を飛ばす
        let 内容 = self.式を解析する()?;
        Some(命令::表示文(内容))
    }

    fn 若し文を解析(&mut self) -> Option<命令> {
        self.advance(); // 「若し」を飛ばす

        let 条件 = self.式を解析する()?;

        if self.current_token != Token::ならば { return None; }
        self.advance();

        if self.current_token != Token::左中括弧 { return None; }
        self.advance();

        let mut 実行内容 = Vec::new();
        while self.current_token != Token::右中括弧 && self.current_token != Token::終端 {
            if let Some(cmd) = self.命令を解析する() {
                実行内容.push(cmd);
            } else {
                self.advance();
            }
        }
        self.advance(); // 「｝」を飛ばす

        Some(命令::若し文 { 条件, 実行内容 })
    }

    fn 式を解析する(&mut self) -> Option<式> {
        let mut 左辺 = match self.current_token {
            Token::数値(n) => 式::数値(n),
            _ => return None,
        };
        self.advance();

        match self.current_token {
            Token::等号 => {
                self.advance();
                let 右辺 = self.式を解析する()?;
                Some(式::比較 {
                    左辺: Box::new(左辺),
                    演算子: "＝".to_string(),
                    右辺: Box::new(右辺),
                })
            }
            Token::不明('+') | Token::不明('-') | Token::不明('*') | Token::不明('/') => {
                let op = if let Token::不明(c) = self.current_token { c } else { '+' };
                self.advance();
                let 右辺 = self.式を解析する()?;
                Some(式::計算 {
                    左辺: Box::new(左辺),
                    演算子: op,
                    右辺: Box::new(右辺),
                })
            }
            _ => Some(左辺), // 記号がない場合は単一の数値を返す
        }
    }
}