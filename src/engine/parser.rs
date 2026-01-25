// src/engine/parser.rs
use crate::engine::ast::{命令, 式};
use crate::engine::lexer::{Lexer, Token};

pub struct Parser {
    lexer: Lexer,
    current_token: Token,
}

impl Parser {
    pub fn new(mut lexer: Lexer) -> Self {
        let first_token = lexer.次のトークンを出す();
        Self { lexer, current_token: first_token }
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
            Token::もし => self.もし文を解析(),
            Token::表示 => self.表示文を解析(),
            Token::記録 => { self.advance(); Some(命令::記録文) }
            Token::送信 => { self.advance(); Some(命令::送信文) }
            _ => None,
        }
    }

    fn 表示文を解析(&mut self) -> Option<命令> {
        self.advance();
        let 内容 = self.式を解析する()?;
        Some(命令::表示文(内容))
    }

    fn もし文を解析(&mut self) -> Option<命令> {
        self.advance(); // 「もし」
        let 条件 = self.式を解析する()?;

        if self.current_token == Token::ならば { self.advance(); }
        if self.current_token == Token::左中括弧 { self.advance(); }

        let mut 実行内容 = Vec::new();
        while self.current_token != Token::終わり 
            && self.current_token != Token::右中括弧 
            && self.current_token != Token::さもなくば 
            && self.current_token != Token::終端 
        {
            if let Some(cmd) = self.命令を解析する() { 実行内容.push(cmd); } 
            else { self.advance(); }
        }

        if self.current_token == Token::右中括弧 { self.advance(); }

        let mut さもなくば内容 = Vec::new();
        if self.current_token == Token::さもなくば {
            self.advance();
            if self.current_token == Token::左中括弧 { self.advance(); }
            while self.current_token != Token::終わり 
                && self.current_token != Token::右中括弧 
                && self.current_token != Token::終端 
            {
                if let Some(cmd) = self.命令を解析する() { さもなくば内容.push(cmd); }
                else { self.advance(); }
            }
            if self.current_token == Token::右中括弧 { self.advance(); }
        }

        if self.current_token == Token::終わり { self.advance(); }

        Some(命令::もし文 { 条件, 実行内容, さもなくば: さもなくば内容 })
    }

    fn 式を解析する(&mut self) -> Option<式> { self.比較の解析() }

    fn 比較の解析(&mut self) -> Option<式> {
        let mut 左辺 = self.加減の解析()?;
        while self.current_token == Token::等号 {
            self.advance();
            let 右辺 = self.加減の解析()?;
            左辺 = 式::比較 { 左辺: Box::new(左辺), 演算子: "＝".to_string(), 右辺: Box::new(右辺) };
        }
        Some(左辺)
    }

    fn 加減の解析(&mut self) -> Option<式> {
        let mut 左辺 = self.乗除の解析()?;
        while self.current_token == Token::加算 || self.current_token == Token::減算 {
            let op = if self.current_token == Token::加算 { '+' } else { '-' };
            self.advance();
            let 右辺 = self.乗除の解析()?;
            左辺 = 式::計算 { 左辺: Box::new(左辺), 演算子: op, 右辺: Box::new(右辺) };
        }
        Some(左辺)
    }

    fn 乗除の解析(&mut self) -> Option<式> {
        let mut 左辺 = match &self.current_token {
            Token::数値(n) => { let val = 式::数値(*n); self.advance(); val }
            Token::文字列(s) => { let val = 式::文字列(s.clone()); self.advance(); val }
            Token::左括弧 => {
                self.advance();
                let 内部 = self.式を解析する()?;
                if self.current_token == Token::右括弧 { self.advance(); }
                内部
            }
            _ => return None,
        };

        while self.current_token == Token::乗算 || self.current_token == Token::除算 {
            let op = if self.current_token == Token::乗算 { '*' } else { '/' };
            self.advance();
            let 右辺 = match &self.current_token {
                Token::数値(n) => { let val = 式::数値(*n); self.advance(); val }
                Token::左括弧 => {
                    self.advance();
                    let 内部 = self.式を解析する()?;
                    if self.current_token == Token::右括弧 { self.advance(); }
                    内部
                }
                _ => return None,
            };
            左辺 = 式::計算 { 左辺: Box::new(左辺), 演算子: op, 右辺: Box::new(右辺) };
        }
        Some(左辺)
    }
}