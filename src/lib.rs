pub mod structs;

use std::{fs, path::PathBuf, process::Command};
use structs::*;

/// Quit the program with a message
fn quit_program(message: &str) -> ! {
    println!("{}", message);
    std::process::exit(1);
}

/// Get the destination path from the args or the windows desktop
fn get_destination_path(args: &Cli) -> PathBuf {
    let win_username = match Command::new("/mnt/c/Windows/System32/cmd.exe")
        .args(["/c", "echo %USERNAME%"])
        .output()
    {
        Ok(output) => {
            let name = String::from_utf8_lossy(&output.stdout);
            name.trim().to_string()
        }
        Err(_) => quit_program("Failed to fetch current Windows Username"),
    };

    let mut path = PathBuf::from(format!(
        "/mnt/c/Users/{}/{}/",
        win_username,
        if args.use_downloads {
            "Downloads"
        } else {
            "Desktop"
        }
    ));

    if args.name.is_some() {
        let name = args.name.as_ref().unwrap();
        if args.origin.is_dir() {
            path.push(name);
            return path;
        }

        let extension = args.origin.extension();
        if extension.is_none() {
            path.push(name);
            return path;
        }

        let extension = match extension.unwrap().to_str() {
            Some(extension) => extension,
            None => quit_program("Failed to get extension!"),
        };

        if name.ends_with(extension) {
            path.push(name);
            return path;
        }

        path.push(format!("{}.{}", name, extension));
    } else {
        path.push(args.origin.file_name().unwrap());
    }

    path
}

/// Check if the origin and destination paths are valid and return them
fn check_paths(args: &Cli) -> (PathBuf, PathBuf) {
    let destination_path = get_destination_path(&args);

    if fs::metadata(&args.origin).is_err() {
        quit_program("No such file or directory found!");
    }

    if fs::metadata(&destination_path).is_ok() && !args.force {
        quit_program("Destination file already exists! Use -f to force action.");
    }

    let orgin_path = match args.origin.canonicalize() {
        Ok(path) => path,
        Err(_) => quit_program("Failed to get origin path!"),
    };

    (orgin_path, destination_path)
}

fn copy(origin_path: &PathBuf, destination_path: &PathBuf) {
    let metadata = match fs::metadata(&origin_path) {
        Ok(metadata) => metadata,
        Err(_) => quit_program("Failed to get origin metadata!"),
    };

    if metadata.is_dir() {
        if let Some(_) = fs::metadata(&destination_path).ok() {
            if let Err(_) = fs::remove_dir_all(&destination_path) {
                quit_program(
                    format!(
                        "Failed to remove directory at {}!",
                        destination_path.display()
                    )
                    .as_str(),
                );
            }
        }

        if let Err(_) = fs::create_dir(&destination_path) {
            quit_program(
                format!(
                    "Failed to create directory at {}!",
                    destination_path.display()
                )
                .as_str(),
            );
        }

        let entries = match fs::read_dir(&origin_path) {
            Ok(entries) => entries,
            Err(_) => quit_program("Failed to read origin directory!"),
        };

        for entry in entries {
            let entry = match entry {
                Ok(entry) => entry,
                Err(_) => quit_program("Failed to read origin directory!"),
            };

            let path = entry.path();

            let file_name = match path.file_name() {
                Some(file_name) => file_name,
                None => quit_program("Failed to get file name!"),
            };

            let destination_path = destination_path.join(file_name);

            copy(&path, &destination_path);
        }
    } else {
        if let Err(_) = fs::copy(&origin_path, &destination_path) {
            quit_program(
                format!(
                    "Failed to copy file to destination {}!",
                    destination_path.display()
                )
                .as_str(),
            );
        }
    }
}

/// Handle the copy action
pub fn handle_copy(args: Cli) {
    let (origin_path, destination_path) = check_paths(&args);
    copy(&origin_path, &destination_path);
}

/// Handle the move action
pub fn handle_move(args: Cli) {
    let (origin_path, destination_path) = check_paths(&args);
    copy(&origin_path, &destination_path);

    if origin_path.is_dir() {
        if let Err(e) = fs::remove_dir_all(&origin_path) {
            println!("{:?}", e);
            quit_program(format!("Failed to remove {}!", origin_path.display()).as_str());
        }
    } else {
        if let Err(e) = fs::remove_file(&origin_path) {
            println!("{:?}", e);
            quit_program(format!("Failed to remove {}!", origin_path.display()).as_str());
        }
    }
}
