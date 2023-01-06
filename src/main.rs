#![allow(unused_results,unused_must_use)]

use clap::Parser;
use std::path::Path;
use std::fs::{read_dir, copy, create_dir_all};
use std::io::Result;
use colorful::Color;
use colorful::Colorful;
use dialoguer::Input;
use std::process::Command;

#[derive(Parser)]
struct Cli {
    name: std::path::PathBuf
}

fn copy_recursively(source: impl AsRef<Path>, destination: impl AsRef<Path>) -> Result<()> {
    create_dir_all(&destination)?;
    for entry in read_dir(source)? {
        let entry = entry?;
        let filetype = entry.file_type()?;
        if filetype.is_dir() {
            copy_recursively(entry.path(), destination.as_ref().join(entry.file_name()))?;
        } else {
            copy(entry.path(), destination.as_ref().join(entry.file_name()))?;
        }
    }
    Ok(())
}

fn main() {
    println!("{}", "      _____                _           _______ _____                         \n     / ____|              | |         |__   __|_   _|      /\\                \n    | |     _ __ ___  __ _| |_ ___ ______| |    | |______ /  \\   _ __  _ __  \n    | |    | '__/ _ \\/ _` | __/ _ \\______| |    | |______/ /\\ \\ | '_ \\| '_ \\ \n    | |____| | |  __/ (_| | ||  __/      | |   _| |_    / ____ \\| |_) | |_) |\n     \\_____|_|  \\___|\\__,_|\\__\\___|      |_|  |_____|  /_/    \\_\\ .__/| .__/ \n                                                                | |   | |    \n                                                                |_|   |_|    \n".gradient(Color::Cyan));
    let args = Cli::parse();
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

    copy_recursively(&Path::new("template"), &args.name);
    println!("{}", "\nNew TI App Created!".bold());
}