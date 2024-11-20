use std::path::Path;
use std::process::Command;

use colored::*;

pub fn guess_mime_type(path: &Path) -> Result<String, String> {
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
        panic!(
            "{} {}",
            "Failed to parse `file` output as UTF-8:".red(),
            error
        );
    });

    Ok(mime_type.trim().replace("/", "_").to_string())
}
