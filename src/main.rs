use std::fs;
use std::process::Command;
use std::path::Path;
use std::env;

fn main() {
    let current_dir = Path::new(".");
    let do_work = env::args().any(|arg| arg == "--do-work");

    let entries = match fs::read_dir(current_dir) {
        Ok(entries) => entries,
        Err(error) => {
            println!("Error reading directory: {}", error);
            return;
        }
    };

    for entry in entries {
        let entry = match entry {
            Ok(entry) => entry,
            Err(error) => {
                println!("Error processing entry: {}", error);
                continue;
            }
        };

        let path = entry.path();
        let file_name = path.file_name().unwrap().to_str().unwrap();

        if file_name == "." || file_name == ".." || file_name == "cleaner.rb" {
            continue;
        }

        let mime_type = match guess_mime_type(&path) {
            Ok(mime_type) => mime_type,
            Err(error) => {
                println!("Error guessing MIME type: {}", error);
                continue;
            }
        };

	// let, let seems.... incorrect?
        let type_directory = mime_type.replace("/", "_");
        let type_directory = Path::new(&type_directory);
        if !type_directory.exists() {
           if do_work {
                match fs::create_dir(type_directory) {
                    Ok(_) => println!("[+] making directory '{}'", type_directory.display()),
                    Err(error) => println!("Error creating directory: {}", error),
                }
            } else {
                println!("[-] skipping make directory '{}'", type_directory.display())
           }
        }

        // it should be getting string fixed inside guess_mime_type()
        if mime_type != "inode_directory" {
            let destination = type_directory.join(file_name);
            if do_work {
                match fs::rename(&path, &destination) {
                    Ok(_) => println!("[-] moving {} => {}", path.display(), destination.display()),
                    Err(error) => println!("Error moving file: {}", error),
                }
            } else {
                println!("[ ] not moving {} => {}", path.display(), destination.display());
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
            panic!("Failed to run `file` command: {}", error);
        });

    if !output.status.success() {
        return Err(format!("`file` command exited with error code: {}", output.status));
    }

    let mime_type = String::from_utf8(output.stdout)
        .unwrap_or_else(|error| {
            panic!("Failed to parse `file` output as UTF-8: {}", error);
        });

    Ok(mime_type.trim().replace("/", "_"))
}
