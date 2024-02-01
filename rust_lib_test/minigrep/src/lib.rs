use std::error::Error;
use std::fs;
use std::env;


pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    let contents = fs::read_to_string(config.filename)?;

    let results = if config.case_sensitive {
        search(&config.query, &contents)//作为if的返回值不添加分号
    } else {
        search_case_insensitive(&config.query, &contents)
    };

    for line in results {
        println!("{}", line);
    }
    Ok(())
}

pub struct Config {
    pub query: String,
    pub filename: String,
    pub case_sensitive: bool,
} 

impl Config {
    //pub fn new(args: &[String]) -> Result<Config, &'static str> {
    pub fn new(mut args: std::env::Args) -> Result<Config, &'static str> {
        if args.len() < 3 {
            return Err("你的输入格式有误：" );
        }

        args.next();

        let query = match args.next() {
            Some(arg) => arg,
            None => return Err("Didn't get a query string"),
        };
        let filename = match args.next() {
            Some(arg) => arg, 
            None => return Err("Didn't get a query string"),
        };



        //这里的 case_sensitive可以认为只要他出现就不区分大小写，不出现就区分大小写
        //env::var函数返回Result,如果case_sensitive设置了，就会包裹在OK这个变体里，否则就会返回err变体
        //is_err用于检查Result是否发生错误
        let case_sensitive = env::var("CASE_INSENSITIVE").is_err();//用于设置环境变量

        Ok(Config { query, filename, case_sensitive })
    }
}

pub fn search<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    // let mut results = Vec::new();

    // for line in contents.lines() {
    //     if line.contains(query) {
    //         results.push(line);
    //     }
    // }
    // results

    contents.lines()
        .filter(|line| line.contains(query))
        .collect()
}

pub fn search_case_insensitive<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    // let mut results = Vec::new();
    // let query = query.to_lowercase();//拥有所有权的string

    // for line in contents.lines() {
    //     if line.to_lowercase().contains(&query) {
    //         results.push(line);
    //     }
    // }
    // results

    contents.lines()
        .filter(|line| line.contains(query))
        .collect()
    
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
//     fn one_result() {
//         let query = "duct";
//         let contents = "\
// Rust:
// safe, fast, productive.
// Pick three.";

//         assert_eq!(vec!["safe, fast, productive."], search(query,contents))
//     }
    fn case_sensitive() {//不区分大小写
        let query = "duct";
        let contents = "\
Rust:
safe, fast, productive.
Pick three.
Duct tape";

        assert_eq!(vec!["safe, fast, productive."], search(query,contents))
    }

    #[test]
    fn case_insensitive() {
        let query = "rUsT";
        let contents =  "\
Rust:
safe, fast, productive.
Pick three.
Trust me.";

        assert_eq!(
            vec!["Rust:", "Trust me."], 
            search_case_insensitive(query, contents)
        )
    }

}