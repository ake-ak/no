use std::env;
use std::fs;
use std::path::{Path, PathBuf};
use std::process::{Command, exit};
use std::time::SystemTime;

// --- COOL STYLING CONSTANTS ---
const C_RESET: &str = "\x1b[0m";
const C_BOLD: &str = "\x1b[1m";
const C_RED: &str = "\x1b[31m";
const C_GREEN: &str = "\x1b[32m";
const C_YELLOW: &str = "\x1b[33m";
const C_CYAN: &str = "\x1b[36m";

fn print_success(msg: &str) {
    println!("{}[OK]{} {}", C_GREEN, C_RESET, msg);
}

fn print_warn(msg: &str) {
    println!("{}[WARN]{} {}", C_YELLOW, C_RESET, msg);
}

fn print_error(msg: &str) {
    eprintln!("{}[ERROR]{} {}", C_RED, C_RESET, msg);
}

fn main() {
    let args: Vec<String> = env::args().collect();

    // 1. Check for system-critical commands (The Shield)
    if args.iter().any(|arg| arg == "/" || arg == "/*") {
        println!("\n{}🚨 STOP! SYSTEM PROTECTION TRIGGERED 🚨{}", C_RED, C_RESET);
        println!("The tool 'no' blocked an attempt to delete root directories.");
        exit(1);
    }

    // 2. Handle subcommands
    if args.len() > 1 {
        match args[1].as_str() {
            "d-list" => {
                show_history();
                return;
            }
            "recover" => {
                if args.len() < 3 {
                    print_error("Please specify a filename to recover.");
                } else {
                    println!("{}🔄 Searching for file...{}", C_CYAN, C_RESET);
                    // Recovery logic here
                }
                return;
            }
            _ => {}
        }
    }

    // 3. Main rm replacement logic
    if args.len() < 2 {
        println!("{}no{} v0.1.0 - The Safe rm", C_BOLD, C_RESET);
        println!("Usage: rm <file>");
        return;
    }

    move_to_trash(&args[1..]);
}

fn move_to_trash(files: &[String]) {
    let trash_dir = dirs::home_dir().unwrap().join("no-trash-sp");
    
    if !trash_dir.exists() {
        fs::create_dir_all(&trash_dir).expect("Failed to create trash folder");
    }

    for file in files {
        if file.starts_with('-') { continue; } // Skip flags like -rf

        let path = Path::new(file);
        if path.exists() {
            let dest = trash_dir.join(path.file_name().unwrap());
            
            match fs::rename(path, &dest) {
                Ok(_) => {
                    print_success(&format!("Moved {}{}{} to trash.", C_BOLD, file, C_RESET));
                }
                Err(e) => print_error(&format!("Could not move {}: {}", file, e)),
            }
        } else {
            print_warn(&format!("File {} does not exist.", file));
        }
    }
}

fn show_history() {
    println!("\n{}📜 DELETION HISTORY (Trash Bin){}", C_CYAN, C_RESET);
    println!("----------------------------------");
    let trash_dir = dirs::home_dir().unwrap().join("no-trash-sp");
    
    if let Ok(entries) = fs::read_dir(trash_dir) {
        for entry in entries.flatten() {
            if let Ok(name) = entry.file_name().into_string() {
                println!("  🗑️  {}", name);
            }
        }
    }
    println!("----------------------------------\n");
}