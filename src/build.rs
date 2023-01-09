use std::fs::read;

use colorful::Colorful;
use serde_json::from_str;
use serde::Deserialize;

#[derive(Debug, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct Config {
    src: String,
    main: String,
    outfile: String
}

pub fn get_config() -> Config {
    let content = read("ticonfig.json").expect(&format!("\n{} {}\n", "✗".red(), "Folder already populated.".bold()));
    let content: Config = from_str(std::str::from_utf8(&content).expect(&format!("\n{} {}\n", "✗".red(), "Error parsing ticonfig.json.".bold()))).expect(&format!("\n{} {}\n", "✗".red(), "Invalid ticonfig.json schema.".bold()));

    return content;
}

pub fn build() {
    let config: Config = get_config();
    
}