#![allow(unused_results,unused_must_use)]

mod includes;
use clap::Parser;
use std::fs::{create_dir, File};
use std::io::Write;
use std::path::Path;
use colorful::Color;
use colorful::Colorful;
use dialoguer::Input;
use std::process::Command;

#[derive(Parser)]
struct Cli {
    name: std::path::PathBuf
}

fn create_file(static_contents: &'static str, name: &str, filename: &str) {
    let mut p = name.to_string();
    p.push('/');
    p.push_str(filename);
    let mut c = File::create(p).expect(&format!("❌ Failed to create {}", &filename)); // ok to panic because this is for built-ins.
    let _ = c.write(static_contents.as_bytes());
}

fn main() {
    let args = Cli::parse();

    println!("{}", "      _____                _           _______ _____                         \n     / ____|              | |         |__   __|_   _|      /\\                \n    | |     _ __ ___  __ _| |_ ___ ______| |    | |______ /  \\   _ __  _ __  \n    | |    | '__/ _ \\/ _` | __/ _ \\______| |    | |______/ /\\ \\ | '_ \\| '_ \\ \n    | |____| | |  __/ (_| | ||  __/      | |   _| |_    / ____ \\| |_) | |_) |\n     \\_____|_|  \\___|\\__,_|\\__\\___|      |_|  |_____|  /_/    \\_\\ .__/| .__/ \n                                                                | |   | |    \n                                                                |_|   |_|    \n".gradient(Color::Cyan));
    let git_installed = Command::new("cmd")
            .args(["git --ver"])
            .output();
    
    if git_installed.is_ok() {
        let input: String = Input::new()
        .with_prompt("Initialize Git Repository? (Y)")
        .default("Y".into())
        .interact_text().unwrap();

        if input.to_lowercase().trim() == "y" || input.to_lowercase().trim() == "yes" {
            let result = Command::new("cmd")
                .args([format!("cd {} && git init", &args.name.display())])
                .output();
            if result.is_err() {
                println!("❌ Git repository failed");
            }
        }
    }

    create_dir(&args.name);

    create_file(&includes::TICONFIG, &args.name.to_str().unwrap(), "ticonfig.json");
    create_dir(&Path::new(&format!("{}/src", &args.name.display())));
    create_file(&includes::KEYPRESS, &args.name.to_str().unwrap(), "src/KEYPRESS.8xp");
    create_file(&includes::MAIN, &args.name.to_str().unwrap(), "src/MAIN.8xp");
    create_file(&includes::UPDATE, &args.name.to_str().unwrap(), "src/UPDATE.8xp");
    create_file(&includes::_APP, &args.name.to_str().unwrap(), "src/_APP.8xp");

    println!("{}", "\nNew TI App Created!".bold());
}