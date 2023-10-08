use std::fs::{read_dir, metadata};
use std::io;
use std::path::Path;
use colored::*;
use std::os::unix::fs::PermissionsExt;

fn main() -> Result<(), io::Error> {
    let args: Vec<String> = std::env::args().collect();

    // Parse the command-line arguments.
    let mut hidden = false;
    for arg in args {
        if arg == "--hidden" {
            hidden = true;
        }
    }

    // List the contents of the current directory.
    let dir_entries = read_dir(".")?;

    for dir_entry in dir_entries {
        let dir_entry = dir_entry?;

        // Skip hidden files if the `--hidden` flag is not set.
        if !hidden && is_hidden(&dir_entry.path())? {
            continue;
        }

        // Print the file name in color depending on the file type.
        let file_type = metadata(&dir_entry.path())?.file_type();
        if file_type.is_dir() {
            println!("{}", dir_entry.path().display().to_string().blue());
        } else if file_type.is_file() {
            println!("{}", dir_entry.path().display().to_string().white());
        } else {
            println!("{}", dir_entry.path().display().to_string().yellow());
        }
    }

    Ok(())
}

fn is_hidden(path: &Path) -> Result<bool, io::Error> {
    let metadata = metadata(path)?;
    let permissions = metadata.permissions();

    #[cfg(windows)]
    {
        Ok(permissions.is_hidden())
    }

    #[cfg(not(windows))]
    {
        let mode = permissions.mode();
        if (mode & 0o10000) != 0 {
            Ok(true)
        } else {
            Ok(false)
        }
    }
}
