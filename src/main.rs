use std::{env, process};
use minigrep::Config;

fn main() {
    // argsメソッドで値生成、collectメソッドで任意のコレクションに変換
    //今回は型注釈をしているので、collectメソッドの方向がVec<String>に推論される
    let args: Vec<String> = env::args().collect();
    
    dbg!(&args); //バイナリファイルパスを確認できる

    // 引数解析
    let config = Config::build(env::args()).unwrap_or_else(|err| {
        eprintln!("引数解析の問題：{err}");
        process::exit(1);
    });

    println!("検索ファイル：{}\n検索対象    ：{}", config.target_file_path, config.query);

    if let Err(e) = minigrep::run(config) {
        eprintln!(" アプリケーションエラー：{e}");
        process::exit(1);
    }
}