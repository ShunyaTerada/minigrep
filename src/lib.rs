use std::{fs, error::Error};

pub fn run(config: Config) -> Result<(), Box<dyn Error>>{
    //ファイルを受け取って開き、そのファイルの内容を含むstd::io::Result<String>を返す
    let contents = fs::read_to_string(config.target_file_path)?;
    println!("付属テキスト：\n{contents}");
    Ok(())
}

pub struct Config {
    pub query: String,
    pub target_file_path: String,
}

impl Config {
    pub fn build(args: &[String]) -> Result<Config, &'static str>{
        // 引数の数が足りてるかチェック
        if args.len() < 3 {
            return Err("引数が足りません");
        }
        let query = args[1].clone(); 
        let target_file_path = args[2].clone();
        //args[0]はバイナリーファイルのパスが占有している

        Ok(Config {query, target_file_path})
    }
}
