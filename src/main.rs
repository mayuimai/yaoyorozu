mod engine;

fn main() {
    // 1. 解析したい日本語のソースコード
    let ソースコード = "若し 10 ＝ 10 ならば ｛ 表示 ｝";
    
    // 2. Lexer（字句解析）の準備
    let レキシカ = engine::lexer::Lexer::new(ソースコード);
    
    // 3. Parser（構文解析）の実行
    let mut パーサ = engine::parser::Parser::new(レキシカ);
    let 構文木 = パーサ.解析する();
    
    // 4. 結果を表示（ここで AST：抽象構文木 が見えます！）
    println!("--- 構文解析結果 ---");
    println!("{:#?}", 構文木);

    // main.rs の最後に追記
let 実行機 = engine::evaluator::Evaluator;
実行機.実行(構文木);
}