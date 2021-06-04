use std::io::{self, Write};
use std::path::PathBuf;
use std::{env, fs};

#[derive(Debug)]
struct DirNode {
    children: Vec<DirNode>,
    path: PathBuf,
    name: String
}

impl Default for DirNode {
    fn default() -> Self {
        Self {
            children: Vec::new(),
            path: PathBuf::new(),
            name: String::from("")
        }
    }
}

impl DirNode {
    fn from(dir_name: &PathBuf) -> Self {
        let mut root = DirNode::default();
        root.path = fs::canonicalize(&dir_name).unwrap();
        root.name = match dir_name.file_name() {
            Some(name) => String::from(name.to_str().unwrap()),
            None => String::from("root")
        };

        if root.path.is_dir() {
            for entry in fs::read_dir(dir_name).unwrap() {
                let path = entry.unwrap().path();
                root.children.push(Self::from(&path));
            }
        }

        root
    }

    fn print_self_and_children(&self, indent_level: usize, is_last: Vec<bool>, self_is_last: bool) {
        let start = if indent_level == 0 { "" } else { "│" };
        let mut indent = String::from("");
        let mut next_is_last = is_last.clone();
        next_is_last.push(self_is_last);

        if indent_level != 0 {
            let mut i = 0;

            loop {
                if i == indent_level {
                    break;
                }

                indent.push_str(if is_last[i] { " " } else { "|" });
                indent.push_str(" ");

                i += 1;
            }
        }

        let seperator = if *next_is_last.last().unwrap() { "└─" } else { "├─" };

        println!("{}{}{}{}", start, indent, seperator, self.name);

        drop(start);
        drop(indent);
        drop(seperator);
        
        let next_indent_level = indent_level + 1;
        let num_children = self.children.len();

        for i in 0..num_children {
            self.children[i].print_self_and_children(next_indent_level, next_is_last.clone(), i == num_children - 1);
        }
    }
}

fn main() {
    println!("Current directory: {}", env::current_dir().unwrap().into_os_string().into_string().unwrap());

    let mut start_directory;

    loop {
        print!("Starting directory: ");
        io::stdout().flush().unwrap();
        
        let mut dir = String::new();
        io::stdin().read_line(&mut dir).unwrap();
        dir = dir.replace("\n", "");

        if dir.trim().is_empty() {
            dir = String::from("./");
        }

        start_directory = PathBuf::from(&dir);
        if start_directory.is_dir() {
            break;
        } else {
            println!("{} is an invalid directory", &dir);
        }
    }

    let root = DirNode::from(&start_directory);
    root.print_self_and_children(0, Vec::new(), true);
}
