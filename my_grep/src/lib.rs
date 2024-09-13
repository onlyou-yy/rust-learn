use std::{env, error::Error, fs};

pub fn run(config:Config) -> Result<(),Box<dyn Error>>{
  let file_content: String = fs::read_to_string(config.file_path)?;
  
  let results = if config.ignore_case {
    search_case_insensitive(&config.query, &file_content)
  }else{
    search_case_sensitive(&config.query, &file_content)
  };
  for line in results{
    println!("result:/{line}");
  }
  
  Ok(())
}

pub struct Config {
  query:String,
  file_path:String,
  ignore_case:bool
}

impl Config {
  pub fn new(args:&[String]) -> Config{
      if args.len() < 3 {
         panic!("至少输入两个参数,eg:IGNORE_CASE=1 cargo run -- body poem.txt"); 
      }
      let query: String = args[1].clone();
      let file_path = args[2].clone();
      let ignore_case = env::var("IGNORE_CASE").is_ok();
      Config {query,file_path,ignore_case}
  }
  pub fn build(mut args_iter: impl Iterator<Item = String>) -> Result<Config, &'static str>{
      args_iter.next();

      let query = match args_iter.next() {
          Some(s) => s,
          None => {
            return  Err("请输入查询字符串");
          }
      };
      let file_path: String = match args_iter.next() {
          Some(s) => s,
          None => {
            return  Err("请输入查询文件名");
          }
      };
      let ignore_case = env::var("IGNORE_CASE").is_ok();

      println!("查询字符串:{}",query);
      println!("查询文件:{}",file_path);
      Ok(Config {query,file_path,ignore_case})
  }
}

fn search_case_insensitive<'a>(query:&str, contents: &'a str) -> Vec<&'a str> {
  contents.lines().filter(|line| line.to_lowercase().contains(&query.to_lowercase())).collect()
}

fn search_case_sensitive<'a>(query:&str, contents: &'a str) -> Vec<&'a str> {
  contents.lines().filter(|line| line.contains(&query)).collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case_sensitive() {
        let query = "duct";
        let contents = "\
Rust:
safe, fast, productive.
Pick three.";

        assert_eq!(vec!["safe, fast, productive."], search_case_sensitive(query, contents));
    }

    #[test]
    fn case_insensitive() {
      let query = "rUsT";
      let contents = "\
Rust:
safe, fast, productive.
Pick three.
Trust me.";

      assert_eq!(
          vec!["Rust:", "Trust me."],
          search_case_insensitive(query, contents)
      );
  }
}