use clap::{Parser, Subcommand};
use colored::*;
use serde::{Deserialize, Serialize};
use std::fs;
use std::path::{Path, PathBuf};
use chrono::Local;

#[derive(Parser)]
#[command(name = "no", version = "0.1.0", about = "A safe rm replacement with history and recovery")]
struct Cli {
    #[command(subcommand)]
    command: Option<Commands>,

    /// The file or directory to delete (standard usage)
    #[arg(value_name = "PATH")]
    path: Option<PathBuf>,
}

#[derive(Subcommand)]
enum Commands {
    /// Show the history of deleted files
    DList,
    /// Recover a file from the trash
    Recover { 
        /// The trash_name shown in d-list
        trash_name: String 
    },
    /// Update the 'no' tool to the latest version
    Update,
}

#[derive(Serialize, Deserialize, Debug)]
struct TrashHistory {
    version: u32,
    entries: Vec<TrashEntry>,
}

#[derive(Serialize, Deserialize, Debug)]
struct TrashEntry {
    original_path: String,
    trash_name: String,
    date: String,
}

fn main() {
    let cli = Cli::parse();
    
    // Setup paths: ~/no-trash-sp/
    let home_dir = home::home_dir().expect("Could not find home directory");
    let trash_dir = home_dir.join("no-trash-sp");
    let history_path = trash_dir.join("history.json");

    // Ensure trash folder exists
    if !trash_dir.exists() {
        fs::create_dir_all(&trash_dir).expect("Failed to create trash folder");
    }

    // Handle Subcommands
    if let Some(cmd) = cli.command {
        match cmd {
            Commands::DList => list_history(&history_path),
            Commands::Recover { trash_name } => recover_file(&trash_dir, &history_path, &trash_name),
            Commands::Update => println!("{}", "Update feature: Coming soon via GitHub releases!".yellow()),
        }
        return;
    }

    // Handle standard "no <path>" (the rm replacement)
    if let Some(target) = cli.path {
        move_to_trash(&trash_dir, &history_path, &target);
    } else {
        println!("Usage: no <path> OR no d-list OR no recover <name>");
    }
}

fn move_to_trash(trash_dir: &Path, history_path: &Path, target: &Path) {
    if !target.exists() {
        println!("{}: No such file or directory", target.display().to_string().red());
        return;
    }

    // Protection for Root /
    if target.to_str() == Some("/") {
        println!("{}", "!!! STOP: You are trying to delete ROOT (/) !!!".on_red().white().bold());
        return;
    }

    let timestamp = Local::now().format("%Y-%m-%d_%H-%M-%S").to_string();
    let file_name = target.file_name().unwrap().to_str().unwrap();
    let trash_name = format!("{}_{}", timestamp, file_name);
    let destination = trash_dir.join(&trash_name);

    // Get absolute path for history
    let original_abs_path = fs::canonicalize(target)
        .unwrap_or(target.to_path_buf())
        .display()
        .to_string();

    match fs::rename(target, &destination) {
        Ok(_) => {
            let mut history = load_history(history_path);
            history.entries.push(TrashEntry {
                original_path: original_abs_path,
                trash_name,
                date: timestamp,
            });
            save_history(history_path, &history);
            println!("{} moved to {}", file_name.green(), "no-trash-sp".cyan());
        }
        Err(e) => println!("Error moving file: {}", e),
    }
}

fn list_history(history_path: &Path) {
    let history = load_history(history_path);
    if history.entries.is_empty() {
        println!("Trash is empty.");
        return;
    }

    println!("{:<25} | {:<20} | {}", "DATE", "TRASH NAME", "ORIGINAL PATH");
    println!("{}", "-".repeat(80));
    for entry in history.entries {
        println!("{:<25} | {:<20} | {}", entry.date.dimmed(), entry.trash_name.yellow(), entry.original_path.cyan());
    }
}

fn recover_file(trash_dir: &Path, history_path: &Path, name: &str) {
    let mut history = load_history(history_path);
    let mut found_index = None;

    for (i, entry) in history.entries.iter().enumerate() {
        if entry.trash_name == name {
            found_index = Some(i);
            break;
        }
    }

    if let Some(i) = found_index {
        let entry = history.entries.remove(i);
        let from = trash_dir.join(&entry.trash_name);
        let to = Path::new(&entry.original_path);

        match fs::rename(from, to) {
            Ok(_) => {
                save_history(history_path, &history);
                println!("{} {}", "Successfully recovered to:".green(), entry.original_path.bold());
            }
            Err(e) => println!("Failed to recover: {}", e),
        }
    } else {
        println!("{}", "Error: Name not found in history.".red());
    }
}

// JSON Helper Functions
fn load_history(path: &Path) -> TrashHistory {
    if !path.exists() {
        return TrashHistory { version: 1, entries: vec![] };
    }
    let data = fs::read_to_string(path).unwrap_or_else(|_| "{}".to_string());
    serde_json::from_str(&data).unwrap_or(TrashHistory { version: 1, entries: vec![] })
}

fn save_history(path: &Path, history: &TrashHistory) {
    let data = serde_json::to_string_pretty(history).unwrap();
    fs::write(path, data).expect("Failed to save history");
}