// src/engine/lexer.rs

#[derive(Debug, PartialEq, Clone)]
pub enum Token {
    もし,
    ならば,
    さもなくば,
    繰返,
    表示,
    変数,
    終わり,
    記録,
    送信,
    識別子(String),
    文字列(String),
    数値(f64),
    等号,
    大なり, // 🌟 追加！ (＞)
    小なり, // 🌟 追加！ (＜)
    左括弧,
    右括弧,
    左中括弧,
    右中括弧,
    加算,
    減算,
    乗算,
    除算,
    空白,
    不明(char),
    終端,
}

pub struct Lexer {
    input: Vec<char>,
    position: usize,
}

impl Lexer {
    pub fn new(input: &str) -> Self {
        Self {
            input: input.chars().collect(),
            position: 0,
        }
    }

    pub fn 次のトークンを出す(&mut self) -> Token {
        self.空白を飛ばす();

        if self.position >= self.input.len() {
            return Token::終端;
        }

        let ch = self.input[self.position];

        match ch {
            '『' => self.read_string(),
            '(' | '（' => { self.read_char(); Token::左括弧 }
            ')' | '）' => { self.read_char(); Token::右括弧 }
            '{' | '｛' => { self.read_char(); Token::左中括弧 }
            '}' | '｝' => { self.read_char(); Token::右中括弧 }
            '=' | '＝' => { self.read_char(); Token::等号 }
            '+' | '＋' => { self.read_char(); Token::加算 }
            '-' | '－' => { self.read_char(); Token::減算 }
            '*' | '＊' => { self.read_char(); Token::乗算 }
            '/' | '／' => { self.read_char(); Token::除算 }
            
            // 🌟 ここに比較記号を追加！
            '>' | '＞' => { self.read_char(); Token::大なり }
            '<' | '＜' => { self.read_char(); Token::小なり }

            _ => {
                if self.is_japanese_alphabetic(ch) || ch.is_ascii_alphabetic() || ch == '_' {
                    let ident = self.read_identifier();
                    match ident.as_str() {
                        "もし" => Token::もし,
                        "ならば" | "なら" => Token::ならば,
                        "さもなくば" | "でなければ" => Token::さもなくば,
                        "繰り返す" | "繰返" => Token::繰返, // 🌟 これを追加！
                        "表示" | "言う" => Token::表示,
                        "終わり" | "以上" | "おわり" => Token::終わり,
                        "記録" => Token::記録,
                        "送信" => Token::送信,
                        "変数" | "箱" | "var" | "let" => Token::変数,
                        _ => Token::識別子(ident),
                    }
                } else if ch.is_ascii_digit() || ('０'..='９').contains(&ch) {
                     Token::数値(self.read_number())
                } else {
                    self.read_char();
                    Token::不明(ch)
                }
            }
        }
    }

    // ... (read_string, read_number, read_identifier などは変更なし) ...
    // ※ 下の関数はそのまま残しておいてください（前回修正した部分です）

    fn read_string(&mut self) -> Token {
        self.read_char(); 
        let start = self.position;
        while self.position < self.input.len() && self.input[self.position] != '』' {
            self.read_char();
        }
        let s: String = self.input[start..self.position].iter().collect();
        if self.position < self.input.len() {
             self.read_char(); 
        }
        Token::文字列(s)
    }

    fn read_number(&mut self) -> f64 {
        let start = self.position;
        while self.position < self.input.len()
            && (self.input[self.position].is_ascii_digit() 
                || ('０'..='９').contains(&self.input[self.position])
                || self.input[self.position] == '.' 
                || self.input[self.position] == '．') 
        {
            self.read_char();
        }
        let s: String = self.input[start..self.position].iter().collect();
        s.replace('０', "0").replace('１', "1").replace('２', "2")
         .replace('３', "3").replace('４', "4").replace('５', "5")
         .replace('６', "6").replace('７', "7").replace('８', "8")
         .replace('９', "9").replace('．', ".").parse().unwrap_or(0.0)
    }

    fn read_identifier(&mut self) -> String {
        let start = self.position;
        while self.position < self.input.len()
            && (self.is_japanese_alphabetic(self.input[self.position]) 
                || self.input[self.position].is_ascii_alphabetic() 
                || self.input[self.position].is_ascii_digit()
                || self.input[self.position] == '_')
        {
            self.read_char();
        }
        self.input[start..self.position].iter().collect()
    }

    fn is_japanese_alphabetic(&self, ch: char) -> bool {
        ('一'..='龠').contains(&ch) || 
        ('ぁ'..='ん').contains(&ch) || 
        ('ァ'..='ヶ').contains(&ch) || 
        ch == 'ー'
    }

    fn read_char(&mut self) { self.position += 1; }

    fn 空白を飛ばす(&mut self) {
        while self.position < self.input.len() && self.input[self.position].is_whitespace() {
            self.read_char();
        }
    }
}