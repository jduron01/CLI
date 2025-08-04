mod vfs;

fn main() -> std::io::Result<()> {
    print!("Enter file name to create or load VFS (or \"exit\" to quit): ");
    std::io::Write::flush(&mut std::io::stdout())?;

    let mut file_name = String::new();
    std::io::stdin().read_line(&mut file_name)?;
    let file_name = file_name.trim();

    if file_name != "exit" {
        let root = load_vfs(&file_name)?;
        let mut navigator = vfs::Navigator::new(root);

        loop {
            print!("rustsh:{}> ", navigator.path());
            std::io::Write::flush(&mut std::io::stdout())?;

            let mut command = String::new();
            std::io::stdin().read_line(&mut command)?;
            let command = command.trim();

            match command.split_whitespace().collect::<Vec<&str>>()[..] {
                ["mkdir", name] => navigator.make_directory(name.into()),
                ["cd", path] => {
                    if !navigator.change_directory(path) {
                        println!("Directory not found: {}", path);
                    }
                }
                ["ls"] => navigator.list_contents(),
                ["clear"] => clearscreen::ClearScreen::default()
                    .clear()
                    .expect("Failed to clear screen."),
                ["exit"] => {
                    save_vfs(navigator.root(), &file_name)?;
                    break;
                }
                _ => println!("Unknown command: {}", command),
            }
        }
    }

    Ok(())
}

fn load_vfs(file_path: &str) -> std::io::Result<vfs::Directory> {
    let vfs_path = std::path::Path::new(file_path);

    if !vfs_path.exists() {
        let root = vfs::Directory::new('/'.into());
        save_vfs(root, file_path)?;
    }

    let data = std::fs::read_to_string(file_path)?;
    let vfs = serde_json::from_str(&data)?;

    Ok(vfs)
}

fn save_vfs(vfs: vfs::Directory, file_path: &str) -> std::io::Result<()> {
    let serialized = serde_json::to_string_pretty(&vfs)?;
    std::fs::write(file_path, serialized)?;
    
    Ok(())
}
