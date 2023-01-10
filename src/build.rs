use std::{fs::{read, read_to_string}, path::Path};

use colorful::{Colorful};
use regex::Regex;
use serde_json::from_str;
use serde::Deserialize;

#[derive(Debug, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct Config {
    src: String,
    main: String,
    outfile: String
}

trait Program {
    fn recursive_prgm(&self);
}

impl Program for str {
    fn recursive_prgm(&self) {
        let re = Regex::new(r"/prgm @[a-zA-Z]+\\/[a-zA-Z]+/g").unwrap();
        let string = self;
        for prgms in re.captures_iter(&string) {
            println!("{:?}", prgms);
        }
    }
}

pub fn get_config() -> Config {
    let content = read("ticonfig.json").expect(&format!("\n{} {}\n", "✗".red(), "Folder already populated.".bold()));
    let content: Config = from_str(std::str::from_utf8(&content).expect(&format!("\n{} {}\n", "✗".red(), "Error parsing ticonfig.json.".bold()))).expect(&format!("\n{} {}\n", "✗".red(), "Invalid ticonfig.json schema.".bold()));

    return content;
}

pub fn build() {
    let config: Config = get_config();
    println!("{}", &config.src);
    println!("{:?}", Path::new(".").join(&config.src).join(&config.main));
    let content = read_to_string(Path::new(&config.src).join(&config.main)).expect(&format!("{} {}", "✗".red(), "Could not read main file or does not exist.".bold()));

    content.recursive_prgm();
}