use std::process::Command;

use clap::Parser;
use colorful::Color;
use colorful::Colorful;
use dialoguer::Input;
use create_ti_app::{git_is_installed, generate_files};

#[derive(Parser)]
struct Cli {
    name: std::path::PathBuf
}

fn create_app() {
    let args = Cli::parse();
    let is_empty = &args.name.read_dir();
    if is_empty.is_ok() {
        println!("\n{} {}\n", "✗".red(), "Folder already populated.".bold());
        return;
    }
    println!("{}", "      _____                _           _______ _____                         \n     / ____|              | |         |__   __|_   _|      /\\                \n    | |     _ __ ___  __ _| |_ ___ ______| |    | |______ /  \\   _ __  _ __  \n    | |    | '__/ _ \\/ _` | __/ _ \\______| |    | |______/ /\\ \\ | '_ \\| '_ \\ \n    | |____| | |  __/ (_| | ||  __/      | |   _| |_    / ____ \\| |_) | |_) |\n     \\_____|_|  \\___|\\__,_|\\__\\___|      |_|  |_____|  /_/    \\_\\ .__/| .__/ \n                                                                | |   | |    \n                                                                |_|   |_|    \n".gradient(Color::Cyan));

    if git_is_installed() {
        let input: String = Input::new()
            .with_prompt(&format!(" {} Initialize Git Repository?", "◦".cyan()))
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

    generate_files(&args.name);

    println!("\n{} {}", "✓".green(), "New TI App Created".light_gray().bold());
    println!("   {} {} {} {}", "◦", "Move into directory".dark_gray(), "cd".cyan(), &args.name.display().to_string().cyan());
    println!("   {} {} {}\n", "◦", "Build project".dark_gray(), "ti build".cyan());
}

pub fn exec() {
    create_app();
}