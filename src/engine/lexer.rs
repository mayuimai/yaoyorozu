// lexer.rs

#[derive(Debug, PartialEq, Clone)]
pub enum Token {
    もし,
    ならば,
    さもなくば, // ←追加
    繰返,
    表示,
    終わり,     // ←追加
    識別子(String),
    数値(f64),
    等号,
    左括弧,
    右括弧,
    左中括弧,
    右中括弧,
    空白,
    不明(char),
    終端,
    // ...
    加算, // ＋
    減算, // －
    乗算, // ＊
    除算, // ／
    // ...
    // ... 今までの単語 ...
    記録,   // ← 追加
    送信,   // ← 追加
    // ...
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
    '（' | '(' => { self.read_char(); return Token::左括弧; }
    '）' | ')' => { self.read_char(); return Token::右括弧; }
    '｛' | '{' => { self.read_char(); return Token::左中括弧; }
    '｝' | '}' => { self.read_char(); return Token::右中括弧; }
    '＝' | '=' => { self.read_char(); return Token::等号; }
    // --- カッコを外して、漢字を「減算」に修正しました ---
    '＋' | '+' => { self.read_char(); return Token::加算; }
    '－' | '-' => { self.read_char(); return Token::減算; }
    '＊' | '*' => { self.read_char(); return Token::乗算; }
    '／' | '/' => { self.read_char(); return Token::除算; }
    _ => {}
}

        if ch.is_ascii_digit() {
            return Token::数値(self.read_number());
        }

        if self.is_japanese_alphabetic(ch) {
            let ident = self.read_identifier();
            return match ident.as_str() {
                "もし" => Token::もし,//if
                "ならば" => Token::ならば,//if (){}
                "さもなくば" => Token::さもなくば,// else{}
                "繰返" => Token::繰返,
                "表示" => Token::表示,//echo
                "終わり" => Token::終わり,//endif end
                "記録" => Token::記録,// git
                "送信" => Token::送信,//git
                _ => Token::識別子(ident),
            };
        }

        self.read_char();
        Token::不明(ch)
    }

    fn read_char(&mut self) {
        self.position += 1;
    }

    fn 空白を飛ばす(&mut self) {
        while self.position < self.input.len() && self.input[self.position].is_whitespace() {
            self.read_char();
        }
    }

    fn is_japanese_alphabetic(&self, ch: char) -> bool {
        ('一'..='龠').contains(&ch) || ('ぁ'..='ん').contains(&ch) || ('ァ'..='ヶ').contains(&ch)
    }

    fn read_identifier(&mut self) -> String {
        let start = self.position;
        while self.position < self.input.len() && self.is_japanese_alphabetic(self.input[self.position]) {
            self.read_char();
        }
        self.input[start..self.position].iter().collect()
    }

    fn read_number(&mut self) -> f64 {
        let start = self.position;
        while self.position < self.input.len() && (self.input[self.position].is_ascii_digit() || self.input[self.position] == '.') {
            self.read_char();
        }
        let s: String = self.input[start..self.position].iter().collect();
        s.parse().unwrap_or(0.0)
    }
}