use std::fmt::Debug;

pub trait Orderable {
    fn compare(&self, other: &Self) -> std::cmp::Ordering;
}

#[derive(Debug)]
pub struct Node<T: Orderable> {
    keys: Vec<T>,
    minimum_degree: usize,
    children: Vec<Node<T>>,
    is_leaf: bool,
    next_node: Option<Box<Node<T>>>,
}

impl<T: Orderable + Debug> Node<T> {
    pub fn new(minimum_degree: usize, is_leaf: bool) -> Self {
        let max_keys = 2 * minimum_degree - 1;
        let max_children = 2 * minimum_degree;

        Node {
            keys: Vec::with_capacity(max_keys),
            children: if is_leaf { Vec::new() } else { Vec::with_capacity(max_children) },
            next_node: None,
            minimum_degree,
            is_leaf,
        }
    }

    pub fn display(&self) {
        println!("{:?}", self.keys);

        if !self.is_leaf {
            for child in &self.children {
                child.display();
            }
        }
    }

    pub fn search(&self, key: &T) -> Option<&Node<T>> {
        let mut i = 0;
        let keys_len = self.keys.len();

        while i < keys_len && key.compare(&self.keys[i]) == std::cmp::Ordering::Less {
            i += 1;
        }

        if i < keys_len && key.compare(&self.keys[i]) == std::cmp::Ordering::Equal {
            return Some(self);
        }

        if self.is_leaf {
            return None;
        }

        self.children[i].search(key)
    }

    pub fn split_child(&mut self, i: usize) {
        let minimum_degree = self.minimum_degree;
        let mut new_node = Node::new(minimum_degree, self.children[i].is_leaf);
        let mut child = self.children.remove(i);

        new_node.keys = child.keys.split_off(minimum_degree);
        if !child.is_leaf {
            new_node.children = child.children.split_off(minimum_degree);
        }

        self.keys.insert(i, child.keys.pop().unwrap());
        self.children.insert(i, child);
        self.children.insert(i + 1, new_node);
    }

    pub fn insert_non_full(&mut self, key: T) {
        let mut i = self.keys.len();

        if self.is_leaf {
            while i > 0 && key.compare(&self.keys[i - 1]) == std::cmp::Ordering::Less {
                i -= 1;
            }

            self.keys.insert(i, key);
            return;
        }

        while i > 0 && key.compare(&self.keys[i - 1]) == std::cmp::Ordering::Less {
            i -= 1;
        }

        if self.children[i].keys.len() == 2 * self.minimum_degree - 1 {
            self.split_child(i);

            if key.compare(&self.keys[i]) == std::cmp::Ordering::Greater {
                i += 1;
            }
        }

        self.children[i].insert_non_full(key);
    }
}

struct BPlusTree<T: Orderable> {
    root: Node<T>,
    minimum_degree: usize,
}


impl<T: Orderable + Debug> BPlusTree<T> {
    pub fn new(minimum_degree: usize) -> Self {
        BPlusTree {
            root: Node::new(minimum_degree, true),
            minimum_degree,
        }
    }

    pub fn insert(&mut self, key: T) {
        if self.root.keys.len() == 2 * self.minimum_degree - 1 {
            let new_root = Node::new(self.minimum_degree, false);
            let old_root = std::mem::replace(&mut self.root, new_root);

            self.root.children.push(old_root);
            self.root.split_child(0);
            self.root.insert_non_full(key);

            return;
        }

        self.root.insert_non_full(key);
    }

    fn display(&self) {
        self.root.display();
    }
}
