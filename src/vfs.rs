use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize)]
pub struct File {
    pub name: String,
    pub content: String,
    pub size: usize
}

#[derive(Serialize, Deserialize)]
pub struct Directory {
    pub name: String,
    pub files: Vec<File>,
    pub subdirs: Vec<Directory>
}

pub struct Navigator {
    pub root: Directory,
    pub path_stack: Vec<usize>,
    pub current_path: String
}

impl Navigator {
    pub fn new(root: Directory) -> Self {
        Navigator {
            root,
            path_stack: Vec::new(),
            current_path: "/".to_string(),
        }
    }

    pub fn current_dir(&self) -> &Directory {
        let mut current = &self.root;

        for &idx in &self.path_stack {
            current = &current.subdirs[idx];
        }

        return current;
    }

    pub fn current_dir_mut(&mut self) -> &mut Directory {
        let mut current = &mut self.root;

        for &idx in &self.path_stack {
            current = &mut current.subdirs[idx];
        }

        return current;
    }

    pub fn change_directory(&mut self, dir_name: &str) -> bool {
        match dir_name {
            ".." => {
                        if !self.path_stack.is_empty() {
                            self.path_stack.pop();
                            self.update_current_path();
                            return true;
                        } else {
                            return false;
                        }
                    }
            name => {
                        let current = self.current_dir();
                        
                        if let Some(index) = current.subdirs.iter().position(|d| d.name == name) {
                            self.path_stack.push(index);
                            self.update_current_path();
                            return true;
                        } else {
                            return false;
                        }
                    }
        }
    }

    pub fn update_current_path(&mut self) {
        self.current_path = "/".to_string();
        let mut current = &self.root;

        for &idx in &self.path_stack {
            current = &current.subdirs[idx];
            self.current_path.push_str(&current.name);
            self.current_path.push('/');
        }
    }

    pub fn make_directory(&mut self, name: String) {
        let new_dir = Directory {
            name,
            files: vec![],
            subdirs: vec![],
        };

        self.current_dir_mut().subdirs.push(new_dir);
    }

    pub fn list_contents(&self) {
        let current = self.current_dir();

        if !self.path_stack.is_empty() {
            println!("..");
        }

        for file in &current.files {
            println!("{} (file)", file.name);
        }

        for dir in &current.subdirs {
            println!("{}/", dir.name);
        }
    }
}