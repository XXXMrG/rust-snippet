/// lib for minigrep
/// @author keith gao
use std::env;
use std::error::Error;
use std::fs;

pub struct Config {
    pub query: String,
    pub filename: String,
    pub case_sensitive: bool,
}

// parse args
impl Config {
    pub fn new(args: &[String]) -> Result<Config, &'static str> {
        if args.len() < 3 {
            return Err("not enough arguments");
        }

        let query = args[1].clone();
        let filename = args[2].clone();

        let case_sensitive = if args.len() > 3 {
            args[3].clone().is_empty()
        } else {
            env::var("CASE_INSENSITIVE").is_err()
        };

        Ok(Config {
            query,
            filename,
            case_sensitive,
        })
    }
}

// Ok 返回一个 () 是一种显示表明此函数仅包含副作用处理的常用方式
pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    // ? will return error if the result is Err
    let content = fs::read_to_string(config.filename)?;

    let results = if config.case_sensitive {
        search(&config.query, &content)
    } else {
        search_insensitive(&config.query, &content)
    };

    for line in results {
        println!("{}", line);
    }

    Ok(())
}

/// search query in contents
pub fn search<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    let mut result = Vec::new();

    for line in contents.lines() {
        if line.contains(query) {
            result.push(line);
        }
    }

    result
}

pub fn search_insensitive<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    let query = query.to_lowercase();
    let mut result = Vec::new();

    for line in contents.lines() {
        if line.to_lowercase().contains(&query) {
            result.push(line)
        }
    }

    result
}

/// 在这一部分，我们将遵循测试驱动开发（Test Driven Development, TDD）的模式来逐步增加 minigrep 的搜索逻辑。
/// 这是一个软件开发技术，它遵循如下步骤：
/// 1. 编写一个失败的测试，并运行它以确保它失败的原因是你所期望的。
/// 2. 编写或修改足够的代码来使新的测试通过。
/// 3. 重构刚刚增加或修改的代码，并确保测试仍然能通过。
/// 从步骤 1 开始重复！
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn search_case_sensitive() {
        let query = "duct";
        let contents = "\
Rust:
safe, fast, productive.
Pick three.
Duct tape.";

        assert_eq!(vec!["safe, fast, productive."], search(query, contents));
    }

    #[test]
    fn search_case_insensitive() {
        let query = "rUsT";
        let contents = "\
Rust:
safe, fast, productive.
Pick three.
Trust me.";

        assert_eq!(
            vec!["Rust:", "Trust me."],
            search_insensitive(&query, &contents)
        );
    }
}
