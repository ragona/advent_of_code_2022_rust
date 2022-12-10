const INPUT: &str = include_str!("files/day_7");

type NodeId = usize;

#[derive(Debug, Eq, PartialEq)]
enum FileType {
    Directory,
    File,
}

#[derive(Debug)]
struct Node {
    id: NodeId,
    data: NodeMetadata,
    parent: Option<NodeId>,
    children: Vec<NodeId>,
}

#[derive(Debug)]
struct NodeMetadata {
    name: String,
    size: usize,
    file_type: FileType,
}

impl NodeMetadata {
    fn new_file(name: String, size: usize) -> Self {
        NodeMetadata {
            name,
            size,
            file_type: FileType::File,
        }
    }

    fn new_dir(name: String) -> Self {
        NodeMetadata {
            name,
            size: 0,
            file_type: FileType::Directory,
        }
    }
}

#[derive(Debug)]
enum Commands {
    Cd(String),
    Ls,
}

impl Commands {
    fn new(cmd: &str, arg: &str) -> Self {
        match cmd {
            "ls" => Self::Ls,
            "cd" => Self::Cd(String::from(arg)),
            _ => unimplemented!("unknown command"),
        }
    }
}

#[derive(Debug)]
struct DirectoryTree {
    nodes: Vec<Node>,
    working_dir: NodeId,
}

impl DirectoryTree {
    fn new() -> Self {
        DirectoryTree {
            nodes: vec![],
            working_dir: 0,
        }
    }

    fn add(&mut self, data: NodeMetadata) -> NodeId {
        let node = Node {
            data,
            parent: Some(self.working_dir),
            children: vec![],
            id: self.nodes.len(),
        };

        let node_id = node.id;
        if node.id != 0 {
            self.nodes[self.working_dir].children.push(node_id);
        }

        self.nodes.push(node);

        node_id
    }

    fn cd(&mut self, dir: &str) {
        if self.nodes.len() == 0 {
            return;
        }

        let pwd = &self.nodes[self.working_dir];

        if dir == ".." {
            if let Some(parent) = pwd.parent {
                self.working_dir = parent;
                return;
            }
        }

        for child in &pwd.children {
            let child = &self.nodes[*child];

            if child.data.name == dir {
                self.working_dir = child.id;
                return;
            }
        }
    }

    fn size(&self, node: NodeId) -> usize {
        let root = &self.nodes[node];
        let mut size = 0;

        let mut stack = vec![root.id];

        while stack.len() > 0 {
            let node = &self.nodes[stack.pop().unwrap()];

            size += node.data.size;

            for child in &node.children {
                stack.push(*child);
            }
        }

        size
    }
}

fn main() {
    part_one();
    part_two();
}

fn part_one() {
    let fs = build_fs();

    let size: usize = fs
        .nodes
        .iter()
        .filter(|node| node.data.file_type == FileType::Directory)
        .map(|node| fs.size(node.id))
        .filter(|size| *size <= 100000)
        .sum();

    dbg!(size);
}

fn part_two() {
    let fs = build_fs();

    let free = 70000000 - &fs.size(0);
    let needed = 30000000;
    let target = needed - free;

    let mut size: Vec<usize> = fs
        .nodes
        .iter()
        .filter(|node| node.data.file_type == FileType::Directory)
        .map(|node| fs.size(node.id))
        .filter(|size| *size >= target)
        .collect();

    size.sort();

    dbg!(size[0]);
}

fn build_fs() -> DirectoryTree {
    let mut fs = DirectoryTree::new();
    fs.add(NodeMetadata::new_dir("/".into()));

    for line in INPUT.lines() {
        if line.chars().nth(0) == Some('$') {
            let cmd = Commands::new(&line[2..4], &line[4..].trim());

            match cmd {
                Commands::Cd(s) => {
                    fs.cd(&s);
                }
                Commands::Ls => (),
            }
        } else {
            let (size_or_dir, name) = line.split_at(line.find(" ").unwrap());
            if size_or_dir == "dir" {
                fs.add(NodeMetadata::new_dir(name.trim().into()));
            } else {
                let size = size_or_dir.parse::<usize>().unwrap();
                fs.add(NodeMetadata::new_file(name.trim().into(), size));
            }
        }
    }

    fs
}
