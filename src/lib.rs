use std::{fs, error::Error, env};

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case_sensitive() {
        let query = "duct";
        //先頭に改行文字を入れないように\を入れている
        let contents = "\
Rust:
safe, fast, productive.
Pick three.
Duct tape";

        assert_eq!(vec!["safe, fast, productive."], search(query, contents));
    }

    #[test]
    fn case_insensitive() {
        let query = "rUst";
        let contents = "\
Rust:
sefe, fast, productive.
Pick three.
Trust me.";

        assert_eq!(
            vec!["Rust:", "Trust me."],
            search_case_insensitive(query, contents)
        );
    }
}

pub struct Config {
    pub query: String,
    pub target_file_path: String,
    pub ignore_case: bool,
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

        //環境変数IGNORE_CASEの有無を確認
        let ignore_case = env::var("IGNORE_CASE").is_ok();

        Ok(Config {
            query,
            target_file_path,
            ignore_case,
        })
    }
}

pub fn run(config: Config) -> Result<(), Box<dyn Error>>{
    //ファイルを受け取って開き、そのファイルの内容を含むstd::io::Result<String>を返す
    let contents = fs::read_to_string(config.target_file_path)?;

    let results = if config.ignore_case {
        search_case_insensitive(&config.query, &contents)
    }else {
        search(&config.query, &contents)
    };

    println!("==============");
    for line in results {
        println!("検索結果    ：{line}");
    }
    
    Ok(())
}


//戻り値のライフタイムはコンテンツが生存している間有効である必要がある
pub fn search<'a>(query: &str, contents: &'a str) -> Vec<&'a str>{
    let mut results = Vec::new();
    // lineメソッドはイテレータを返す
    for line in contents.lines(){
        if line.contains(query){
            results.push(line);
        }
    }

    results
}

//大文字小文字関係なく検索したい
pub fn search_case_insensitive<'a>(query: &str, contents: &'a str) -> Vec<&'a str>{
    //to_lowercaseメソッドは100%正確ではないため、厳密にしたい場合は以下のコードでは不十分
    let query = query.to_lowercase();//変換後はStringになる
    let mut results = Vec::new();

    for line in contents.lines() {
        if line.to_lowercase().contains(&query) {//小文字変換するときにStringになっているため参照記号（&）が必要
            results.push(line);
        }
    }

    results
}



