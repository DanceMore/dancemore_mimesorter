use std::fs;
use std::path::Path;
use std::process::Command;

use clap::CommandFactory;
use clap::Parser;
use colored::*;
use glob::{glob_with, MatchOptions};

#[derive(Parser, Debug)]
#[command(name = "mimesorter", author, version, about = "sort your files by MIME type", long_about = None)]
struct Cli {
    #[clap(long)]
    dry_run: bool,

    #[clap(long)]
    do_work: bool,
}

fn main() {
    let cli = Cli::parse();

    let current_dir = Path::new(".");
    let do_work = cli.do_work;
    let dry_run = cli.dry_run;

    // by default, print help
    if !dry_run && !do_work {
        let mut cmd = Cli::command();
        let _ = cmd.print_help();
        return;
    }

    if dry_run && do_work {
        eprintln!("{}", "those arguments are mutually exclusive and I think you knew that.\n".red());
        let mut cmd = Cli::command();
        let _ = cmd.print_help();
        return;
    }

    if dry_run {
        println!("{}", "[!] dry-run in progress, pass --do-work to organize files based on this preview".yellow());
    }

    // List of excluded files and patterns
    let excluded_patterns: Vec<&str> = vec![
        ".DS_Store", "._*",  // macOS
        "Thumbs.db", "desktop.ini", "$RECYCLE.BIN",  // Windows
        ".directory", ".hidden", ".Trash-*",  // Linux
        "*.swp", "*~",  // Vim/Emacs swap files
        ".lock",  // Lock files
        ".git", ".svn", ".hg", ".bzr",  // Version control directories
        ".idea",  // IntelliJ IDEA project configuration
        ".vscode"  // Visual Studio Code settings
    ];

    // Create MatchOptions to ignore case and allow hidden files
    let match_options = MatchOptions {
        case_sensitive: false,
        require_literal_separator: false,
        require_literal_leading_dot: false,
    };

    let entries = match fs::read_dir(current_dir) {
        Ok(entries) => entries,
        Err(error) => {
            println!("{} {}", "[!] Error reading directory:".red(), error);
            return;
        }
    };

    for entry in entries {
        let entry = match entry {
            Ok(entry) => entry,
            Err(error) => {
                println!("{} {}", "[!] Error processing entry:".red(), error);
                continue;
            }
        };

        let path = entry.path();
        let file_name = path.file_name().unwrap().to_str().unwrap();

        // Check if the file name matches any excluded pattern
        let is_excluded = excluded_patterns.iter().any(|pattern| {
            glob_with(&format!("./{}", pattern), match_options.clone())
                .map_or(false, |mut iter| iter.any(|g| g.unwrap() == path))
        });

        if is_excluded {
            println!("{} {}", "[-] skipping excluded file:".red(), file_name);
            continue;
        }

        if file_name == "." || file_name == ".." {
            continue;
        }

        // don't process directories.
        // they are Named already (and thus semi sorted)
        // we are sorting files only.
        if entry.file_type().unwrap().is_dir() {
            println!("{} {}", "[-] skipping directory:".red(), path.display());
            continue;
        }

        let mime_type = match guess_mime_type(&path) {
            Ok(mime_type) => mime_type,
            Err(error) => {
                println!("{} {}", "Error guessing MIME type:".red(), error);
                continue;
            }
        };

        // let, let seems.... incorrect?
        let type_directory = mime_type.replace("/", "_");
        let type_directory = Path::new(&type_directory);
        if !type_directory.exists() {
            if do_work {
                match fs::create_dir(type_directory) {
                    Ok(_) => println!("{} {}", "[+] making directory '{}'".green(), type_directory.display()),
                    Err(error) => println!("{} {}", "Error creating directory:".red(), error),
                }
            } else {
                println!("{} {}", "[-] skipping make directory '{}'".yellow(), type_directory.display())
            }
        }

        // it should be getting string fixed inside guess_mime_type()
        if mime_type != "inode_directory" {
            let destination = type_directory.join(file_name);
            if do_work {
                match fs::rename(&path, &destination) {
                    Ok(_) => println!("{} {} => {}", "[-] moving".blue(), path.display().to_string().dimmed(), destination.display().to_string().dimmed()),
                    Err(error) => println!("{} {}", "Error moving file:".red(), error),
                }
            } else {
                println!(
                    "{} {} => {}",
                    "[ ] not moving".cyan(),
                    path.display().to_string().dimmed(),
                    destination.display().to_string().dimmed()
                );
            }
        }
    }
}

fn guess_mime_type(path: &Path) -> Result<String, String> {
    let output = Command::new("file")
        .arg("-b")
        .arg("--mime-type")
        .arg(path)
        .output()
        .unwrap_or_else(|error| {
            panic!("{} {}", "Failed to run `file` command:".red(), error);
        });

    if !output.status.success() {
        return Err(format!(
            "{} {}",
            "`file` command exited with error code:".red(),
            output.status
        ));
    }

    let mime_type = String::from_utf8(output.stdout).unwrap_or_else(|error| {
        panic!("{} {}", "Failed to parse `file` output as UTF-8:".red(), error);
    });

    Ok(mime_type.trim().replace("/", "_"))
}
