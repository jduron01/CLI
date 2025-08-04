#[derive(serde::Serialize, serde::Deserialize)]
pub struct File {
    name: String,
    content: String,
    size: usize,
}

#[derive(serde::Serialize, serde::Deserialize)]
pub struct Directory {
    name: String,
    files: Vec<File>,
    subdirs: Vec<Directory>,
}

pub struct Navigator {
    root: Directory,
    path_stack: Vec<usize>,
    path: String,
}

impl Directory {
    pub fn new(name: String) -> Self {
        return Directory {
            name,
            files: Vec::new(),
            subdirs: Vec::new(),
        };
    }
}

impl Navigator {
    pub fn new(root: Directory) -> Self {
        return Navigator {
            root,
            path_stack: Vec::new(),
            path: '/'.into(),
        };
    }

    pub fn root(self) -> Directory {
        return self.root;
    }

    pub fn path(&self) -> &String {
        return &self.path;
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

                if let Some(index) = current
                    .subdirs
                    .iter()
                    .position(|directory| directory.name == name)
                {
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
        self.path = '/'.into();
        let mut current = &self.root;

        for &idx in &self.path_stack {
            current = &current.subdirs[idx];
            self.path.push_str(&current.name);
            self.path.push('/');
        }
    }

    pub fn make_directory(&mut self, name: String) {
        let new_dir = Directory::new(name);
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
            println!("{} (dir)", dir.name);
        }
    }
}
