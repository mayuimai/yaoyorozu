#[derive(Debug)]
pub enum 命令 {
    若し文 {
        条件: 式,
        実行内容: Vec<命令>,
    },
    表示文(式),
}

#[derive(Debug)]
pub enum 式 {
    数値(f64),
    比較 {
        左辺: Box<式>,
        右辺: Box<式>,
    },
}