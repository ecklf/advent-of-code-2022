use aoc_runner_derive::{aoc, aoc_generator};
use std::{
    borrow::{Borrow, BorrowMut},
    cell::RefCell,
    rc::{Rc, Weak},
};

#[derive(Debug, Clone, Copy)]
pub enum FSNodeType {
    Dir,
    File,
}

#[derive(Debug, Clone, Copy)]
pub struct FSNode {
    pub kind: FSNodeType,
    pub name: &'static str,
    pub size: i32,
}

#[derive(Debug)]
pub enum Instruction {
    NavigateUp,
    NavigateIn(String),
    File(FSNode),
}

#[derive(Debug)]
pub struct Node<T> {
    parent: Option<Weak<RefCell<Node<T>>>>,
    children: Vec<Rc<RefCell<Node<T>>>>,
    content: T,
}

pub struct Root<T> {
    pub current: Option<Rc<RefCell<Node<T>>>>,
    pub children: Vec<Rc<RefCell<Node<T>>>>,
}

impl<T> Default for Root<T> {
    fn default() -> Self {
        Root {
            current: None,
            children: Vec::new(),
        }
    }
}

impl<T> Root<T> {
    fn navigate_to_parent(&mut self) {
        if let Some(current_node) = self.current.borrow_mut() {
            let borrowed = current_node.clone();
            let borrowed = borrowed.as_ref().borrow();

            match &borrowed.parent {
                Some(parent) => {
                    let upgraded = parent.upgrade().unwrap();
                    *current_node = upgraded;
                }
                None => {
                    let curr = self.current.borrow_mut();
                    *curr = None
                }
            }
        }
    }

    fn insert(&mut self, content: T) {
        match self.current.borrow_mut() {
            Some(node) => {
                let cloned = node.clone();
                let to_insert = Node {
                    parent: Some(Rc::downgrade(&cloned)),
                    children: Vec::new(),
                    content,
                };

                node.as_ref()
                    .borrow_mut()
                    .children
                    .push(Rc::new(RefCell::new(to_insert)));
            }
            None => {
                let to_insert = Node {
                    parent: None,
                    children: Vec::new(),
                    content,
                };

                self.children.push(Rc::new(RefCell::new(to_insert)));
            }
        };
    }
}

impl Root<FSNode> {
    fn print(&mut self, children: Option<Vec<Rc<RefCell<Node<FSNode>>>>>, depth: Option<i32>) {
        let children = children.unwrap_or_else(|| self.children.clone());
        let depth = match depth {
            Some(d) => d + 1,
            None => 0,
        };

        let padding = (0..=depth).map(|_| "  ").collect::<String>();

        children.iter().for_each(|child| {
            let child = child.as_ref().borrow();
            let debug = format!("{} - {}", padding, child.content.borrow().name);
            println!("{}", debug);

            self.print(Some(child.children.clone()), Some(depth))
        })
    }

    fn loop_through_dirs(&mut self) -> i32 {
        struct FileSizes {
            children: Vec<Rc<RefCell<Node<FSNode>>>>,
            sum: i32,
        }

        impl FileSizes {
            fn get_sum(&mut self, children: Option<Vec<Rc<RefCell<Node<FSNode>>>>>) {
                let children = children.unwrap_or_else(|| self.children.clone());

                let sizes = children.iter().fold(0, |acc, child| {
                    let child = child.as_ref().borrow();

                    if let FSNodeType::File = child.content.kind {
                        return acc + child.content.size;
                    }
                    acc
                });

                self.sum += sizes;

                children.iter().for_each(|child| {
                    let child = child.as_ref().borrow();
                    self.get_sum(Some(child.children.clone()));
                });
            }
        }

        let mut file_sizes = FileSizes {
            children: self.children.clone(),
            sum: 0,
        };

        file_sizes.get_sum(None);
        file_sizes.sum
    }

    fn insert_child(&mut self, content: FSNode) {
        self.insert(content);
        if let Some(current) = self.current.borrow_mut() {
            let mut current = current.as_ref().borrow_mut();
            current.content.size += content.size;
        }

        // if let Some(parent) = &current.parent {
        //     self.update_parent_size(current.content.size);
        // }
    }

    fn update_parent_size(&mut self, size: i32) {}

    fn navigate_to_child(&mut self, name: &str) {
        let find_child = |child: &&Rc<RefCell<Node<FSNode>>>| -> bool {
            let child = child.as_ref().borrow();
            child.content.name == name && child.content.size == 0
        };

        match self.current.borrow_mut() {
            Some(curr) => {
                let borrowed = curr.clone();
                let borrowed = borrowed.as_ref().borrow();

                let found_child = borrowed.children.iter().find(find_child).unwrap();
                *curr = Rc::clone(found_child);
            }
            None => {
                let curr_borrowed = self.current.borrow_mut();

                let found_child = self.children.iter().find(find_child).unwrap();
                *curr_borrowed = Some(Rc::clone(found_child));
            }
        }
    }
}

#[aoc_generator(day7)]
fn input_generator(input: &str) -> Vec<Instruction> {
    let lines = input
        .split('\n')
        .filter_map(|line| {
            if line.starts_with('$') {
                if line.contains("cd") {
                    let (_, cd) = line.split_once(' ').unwrap();
                    let (_, dir) = cd.split_once(' ').unwrap();

                    if dir == "/" {
                        return None;
                    }

                    if dir == ".." {
                        return Some(Instruction::NavigateUp);
                    }

                    return Some(Instruction::NavigateIn(dir.to_owned()));
                }
                None
            } else if line.starts_with("dir") {
                let (_, dir) = line.split_once(' ').unwrap();
                let boxed_str = Box::leak(dir.to_owned().into_boxed_str());

                Some(Instruction::File(FSNode {
                    kind: FSNodeType::Dir,
                    name: boxed_str,
                    size: 0,
                }))
            } else {
                let (size, file) = line.split_once(' ').unwrap();
                let boxed_str = Box::leak(file.to_owned().into_boxed_str());

                Some(Instruction::File(FSNode {
                    kind: FSNodeType::File,
                    name: boxed_str,
                    size: size.parse::<i32>().unwrap(),
                }))
            }
        })
        .collect::<Vec<Instruction>>();
    lines
}

#[aoc(day7, part1)]
pub fn part_one(instructions: &[Instruction]) -> i32 {
    let mut tree = Root::<FSNode>::default();

    for instruction in instructions.iter() {
        match instruction {
            Instruction::File(file) => tree.insert_child(*file),
            Instruction::NavigateUp => {
                tree.navigate_to_parent();
            }
            Instruction::NavigateIn(dir) => {
                tree.navigate_to_child(dir);
            }
        }
    }

    dbg!(tree.children);
    42
}

// #[aoc(day7, part2)]
// pub fn part_two(instructions: &[Instruction]) -> i32 {
//     42
// }
