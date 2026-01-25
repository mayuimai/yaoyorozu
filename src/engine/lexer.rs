#[derive(Debug, Clone, PartialEq)]
pub enum Token {
    数値(f64), // 🌟 ここが修正ポイント！正しい「値」になっています
    文字列(String),
    識別子(String),
    
    // キーワード
    もし, ならば, さもなくば, 繰返,
    表示, 終わり, 記録, 送信,
    変数,
    時刻, 日時, 曜日,

    // 記号
    等号,       // =
    大なり,     // >
    小なり,     // <
    加算,       // +
    減算,       // -
    乗算,       // *
    除算,       // /
    左括弧,     // (
    右括弧,     // )
    左中括弧,   // {
    右中括弧,   // }
    
    終端,       // EOF
}

pub struct Lexer {
    input: Vec<char>,
    pub position: usize,
}

impl Lexer {
    pub fn new(input: &str) -> Self {
        Self {
            input: input.chars().collect(),
            position: 0,
        }
    }

    pub fn 次のトークンを出す(&mut self) -> Token {
        self.空白をスキップ();

        if self.position >= self.input.len() {
            return Token::終端;
        }

        let ch = self.input[self.position];

        // コメント（#）スキップ処理
        if ch == '#' {
            self.コメントをスキップ();
            return self.次のトークンを出す();
        }

        // 数字
        if ch.is_digit(10) || "０１２３４５６７８９".contains(ch) {
            return self.数値を読む();
        }

        // 文字列
        if ch == '「' {
            return self.文字列を読む();
        }

        // 識別子・キーワード
        if Self::is_identifier_start(ch) {
            let ident = self.識別子を読む();
            return match ident.as_str() {
                "もし" => Token::もし,
                "時刻" | "いま" => Token::時刻,
                "日時" | "日付" => Token::日時,
                "曜日" | "ようび" => Token::曜日,
                "ならば" | "なら" => Token::ならば,
                "さもなくば" | "でなければ" => Token::さもなくば,
                "繰り返す" | "繰返" | "ずっと" => Token::繰返,
                "表示" | "言う" => Token::表示,
                "終わり" | "以上" | "おわり" => Token::終わり,
                "記録" | "セーブ" | "コミット" => Token::記録,
                "送信" | "プッシュ" => Token::送信,
                "変数" | "箱" => Token::変数,
                _ => Token::識別子(ident),
            };
        }

        // 記号
        match ch {
            '=' | '＝' => { self.position += 1; Token::等号 },
            '+' | '＋' => { self.position += 1; Token::加算 },
            '-' | 'ー' => { self.position += 1; Token::減算 },
            '*' | '×' => { self.position += 1; Token::乗算 },
            '/' | '÷' => { self.position += 1; Token::除算 },
            '>' | '＞' => { self.position += 1; Token::大なり },
            '<' | '＜' => { self.position += 1; Token::小なり },
            '(' | '（' => { self.position += 1; Token::左括弧 },
            ')' | '）' => { self.position += 1; Token::右括弧 },
            '{' | '｛' => { self.position += 1; Token::左中括弧 },
            '}' | '｝' => { self.position += 1; Token::右中括弧 },
            _ => {
                self.position += 1;
                self.次のトークンを出す()
            }
        }
    }

    fn 空白をスキップ(&mut self) {
        while self.position < self.input.len() {
            let ch = self.input[self.position];
            if ch == ' ' || ch == '\t' || ch == '\n' || ch == '\r' || ch == '　' {
                self.position += 1;
            } else {
                break;
            }
        }
    }

    fn コメントをスキップ(&mut self) {
        while self.position < self.input.len() {
            if self.input[self.position] == '\n' {
                break;
            }
            self.position += 1;
        }
    }

    fn 数値を読む(&mut self) -> Token {
        let start = self.position;
        while self.position < self.input.len() {
            let ch = self.input[self.position];
            if ch.is_digit(10) || ch == '.' || "０１２３４５６７８９．".contains(ch) {
                self.position += 1;
            } else {
                break;
            }
        }
        let s: String = self.input[start..self.position].iter().collect();
        let normalized: String = s.chars().map(|c| match c {
            '０'..='９' => char::from_u32(c as u32 - '０' as u32 + '0' as u32).unwrap(),
            '．' => '.',
            _ => c,
        }).collect();
        let n = normalized.parse().unwrap_or(0.0);
        Token::数値(n)
    }

    fn 文字列を読む(&mut self) -> Token {
        self.position += 1; // 「
        let start = self.position;
        while self.position < self.input.len() && self.input[self.position] != '」' {
            self.position += 1;
        }
        let s: String = self.input[start..self.position].iter().collect();
        if self.position < self.input.len() {
             self.position += 1; // 」
        }
        Token::文字列(s)
    }

    fn 識別子を読む(&mut self) -> String {
        let start = self.position;
        while self.position < self.input.len() {
            let ch = self.input[self.position];
            if Self::is_identifier_char(ch) {
                self.position += 1;
            } else {
                break;
            }
        }
        self.input[start..self.position].iter().collect()
    }

    fn is_identifier_start(ch: char) -> bool {
        !ch.is_digit(10) && !"# 「」=+-*/><(){}\t\n\r　＝＋ー×÷＞＜（）｛｝".contains(ch)
    }

    fn is_identifier_char(ch: char) -> bool {
        !ch.is_whitespace() && !"# 「」=+-*/><(){}\t\n\r　＝＋ー×÷＞＜（）｛｝".contains(ch)
    }
}