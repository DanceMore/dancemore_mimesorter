use std::collections::HashMap;
use std::fs::{self, ReadDir};
use std::path::{Path, PathBuf};

use clap::CommandFactory;
use clap::Parser;
use colored::*;
use glob::{glob_with, MatchOptions};

use mimesorter::guess_mime_type;

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

    // snark off about the impossible
    if dry_run && do_work {
        eprintln!(
            "{}",
            "those arguments are mutually exclusive and I think you knew that.\n".red()
        );
        let mut cmd = Cli::command();
        let _ = cmd.print_help();
        return;
    }

    if dry_run {
        println!(
            "{}",
            "[!] dry-run in progress, pass --do-work to organize files based on this preview"
                .yellow()
        );
    }

    let entries = fs::read_dir(current_dir);
    organize_files(entries, dry_run, do_work);
}

fn organize_files(
    entries: Result<ReadDir, std::io::Error>,
    dry_run: bool,
    do_work: bool,
) -> HashMap<String, Vec<PathBuf>> {
    let mut files_by_mime_type: HashMap<String, Vec<PathBuf>> = HashMap::new();

    // List of excluded files and patterns
    #[rustfmt::skip]
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

    match entries {
        Ok(entries) => {
            for entry in entries {
                match entry {
                    Ok(entry) => {
                        let path = entry.path();
                        let file_name = path.file_name().and_then(|s| s.to_str()).unwrap_or("");

                        if file_name == "." || file_name == ".." {
                            continue;
                        }

                        let is_excluded = excluded_patterns.iter().any(|pattern| {
                            glob_with(&format!("./{}", pattern), match_options.clone())
                                .map_or(false, |mut iter| iter.any(|g| g.unwrap() == path))
                        });

                        if is_excluded || entry.file_type().map_or(true, |ft| ft.is_dir()) {
                            continue;
                        }

                        let mime_type =
                            guess_mime_type(&path).unwrap_or_else(|_| "unknown".to_string());

                        if mime_type != "inode_directory" {
                            files_by_mime_type
                                .entry(mime_type)
                                .or_insert_with(Vec::new)
                                .push(path);
                        }
                    }
                    Err(error) => println!("{} {}", "[!] Error processing entry:".red(), error),
                }
            }
        }
        Err(error) => println!("{} {}", "[!] Error reading directory:".red(), error),
    }

    if do_work {
        // Create directories and move files based on collected data
        for (mime_type, paths) in &files_by_mime_type {
            let type_directory_path = Path::new(mime_type);

            if !type_directory_path.exists() {
                match fs::create_dir(type_directory_path) {
                    Ok(_) => println!(
                        "{} {}",
                        "[+] making directory '{}'".green(),
                        type_directory_path.display()
                    ),
                    Err(error) => println!("{} {}", "Error creating directory:".red(), error),
                }
            }

            for path in paths {
                let file_name = path.file_name().unwrap();
                let destination = type_directory_path.join(file_name);

                match fs::rename(path, &destination) {
                    Ok(_) => println!(
                        "{} {} => {}",
                        "[-] moving".blue(),
                        path.display().to_string().dimmed(),
                        destination.display().to_string().dimmed()
                    ),
                    Err(error) => println!("{} {}", "Error moving file:".red(), error),
                }
            }
        }
    } else if dry_run {
        // Print preview of what will be done
        for (mime_type, paths) in &files_by_mime_type {
            let type_directory_path = Path::new(mime_type);

            println!(
                "{} {}",
                "[-] skipping make directory '{}'".yellow(),
                type_directory_path.display()
            );

            for path in paths {
                let file_name = path.file_name().unwrap();
                let destination = type_directory_path.join(file_name);

                println!(
                    "{} {} => {}",
                    "[ ] not moving".cyan(),
                    path.display().to_string().dimmed(),
                    destination.display().to_string().dimmed()
                );
            }
        }
    }

    files_by_mime_type
}
