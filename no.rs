use std::process;
use std::io::{self, Write};

fn main() {
    let args: Vec<String> = std::env::args().collect();

    // Check if the user is targeting root
    if args.contains(&"/".to_string()) && args.iter().any(|a| a == "-rf" || a == "-fr") {
        println!("\x1b[93m[!] WARNING: YOU ARE TRYING TO DELETE THE SYSTEM ROOT (/)\x1b[0m");
        print!("This action is blocked by 'no'. Type 'I UNDERSTAND' to override: ");
        
        io::stdout().flush().unwrap();
        let mut input = String::new();
        io::stdin().read_line(&mut input).unwrap();

        if input.trim() != "I UNDERSTAND" {
            println!("Aborted. No changes made.");
            process::exit(1);
        }
    }

    // Logic for moving to ~/.no_trash goes here...
    println!("Moving files to safe storage...");
}