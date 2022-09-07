use std::{
    env, fs,
    io::{self, Write},
    path::{Path, PathBuf},
};

#[derive(Debug)]
struct DirNode {
    children: Vec<DirNode>,
    name: String,
}

impl DirNode {
    fn from_dir<T: AsRef<Path>>(dir_name: T) -> io::Result<Self> {
        let path = fs::canonicalize(&dir_name)?;
        let name = match dir_name.as_ref().file_name() {
            Some(name) => name.to_string_lossy().to_string(),
            None => String::from("root"),
        };

        let children = if path.is_dir() {
            fs::read_dir(dir_name)?.flatten().filter_map(|entry| Self::from_dir(entry.path()).ok()).collect()
        } else {
            Vec::new()
        };

        Ok(Self { children, name })
    }

    fn print_self_and_children(&self, indent_level: usize, mut is_last: Vec<bool>, self_is_last: bool) {
        let mut indent = String::new();

        if indent_level != 0 {
            let mut i = 0;

            loop {
                if i == indent_level {
                    break;
                }

                indent.push(if is_last[i] { ' ' } else { '|' });
                indent.push(' ');

                i += 1;
            }
        }

        let seperator = if self_is_last { "└─" } else { "├─" };
        is_last.push(self_is_last);

        println!("{indent}{seperator}{}", self.name);

        for (i, child) in self.children.iter().enumerate() {
            child.print_self_and_children(indent_level + 1, is_last.clone(), i == self.children.len() - 1);
        }
    }
}

fn main() -> io::Result<()> {
    println!("Current directory: {}", env::current_dir()?.to_string_lossy());

    let mut start_directory;

    loop {
        print!("Starting directory: ");
        io::stdout().flush()?;

        let mut dir = String::new();
        io::stdin().read_line(&mut dir)?;
        dir = dir.replace('\n', "");
        let mut trim_dir = dir.trim();

        if trim_dir.is_empty() {
            trim_dir = "./";
        }

        println!("{trim_dir}");
        start_directory = PathBuf::from(trim_dir);
        if start_directory.is_dir() {
            break;
        } else {
            println!("{trim_dir} is an invalid directory");
        }
    }

    let root = DirNode::from_dir(&start_directory)?;
    root.print_self_and_children(0, Vec::new(), true);

    Ok(())
}
