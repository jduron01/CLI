mod vfs;

use vfs::{File, Directory, Navigator};
use std::io::{self, Write};
use std::fs;
use std::path::Path;
use serde_json;
use clearscreen::{self, ClearScreen};

fn main() -> std::io::Result<()> {
    print!("Enter file name to load VFS (or \"exit\" to quit): ");
    io::stdout().flush()?;

    let mut file_name = String::new();
    io::stdin().read_line(&mut file_name)?;
    file_name = file_name.trim().to_string();

    if file_name != "exit" {
        let root = load_vfs(&file_name)?;
        let mut navigator = Navigator::new(root);
        
        loop {
            print!("rustsh:{}> ", navigator.current_path);
            io::stdout().flush()?;

            let mut command = String::new();
            io::stdin().read_line(&mut command)?;
            let command = command.trim();

            match command.split_whitespace().collect::<Vec<_>>().as_slice() {
                ["mkdir", name] => navigator.make_directory(name.to_string()),
                ["cd", path] => {
                                    if !navigator.change_directory(path) {
                                        println!("Directory not found: {}", path);
                                    }
                                },
                ["ls"] => navigator.list_contents(),
                ["clear"] => ClearScreen::default().clear().expect("Failed to clear screen."),
                ["exit"] => {
                                save_vfs(&navigator.root, &file_name)?;
                                break;
                            },
                _ => println!("Unknown command: {}", command),
            }
        }
    }

    Ok(())
}

fn save_vfs(vfs: &Directory, file_path: &str) -> std::io::Result<()> {
    let serialized = serde_json::to_string_pretty(vfs)?;
    fs::write(file_path, serialized)?;

    Ok(())
}

fn load_vfs(file_path: &str) -> std::io::Result<Directory> {
    let vfs_path = Path::new(file_path);

    if !vfs_path.exists() {
        let root = Directory {
            name: String::from("/"),
            files: vec![],
            subdirs: vec![]
        };

        save_vfs(&root, file_path)?;
    }

    let data = fs::read_to_string(file_path)?;
    let vfs = serde_json::from_str(&data)?;

    Ok(vfs)
}