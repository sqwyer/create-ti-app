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
        let re = Regex::new(r"prgm(?P<name>[a-zA-z]+)").unwrap();
        let string = &self;
        // println!("{}", &string);
        // println!("{:?}", re.captures(&string));
        for caps in re.captures_iter(&string) {
            println!("{:?}", &caps.name("name"));
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
    let mut mainfile = std::env::current_dir().unwrap().to_str().unwrap().to_owned();
    mainfile.push_str(&config.src);
    mainfile.push_str(&config.main);

    let content = read_to_string(Path::new(&mainfile)).expect("Error reading main file.");

    content.recursive_prgm();
}