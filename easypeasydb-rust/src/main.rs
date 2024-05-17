#[allow(dead_code)]
use std::rc::Rc;
use std::{borrow::BorrowMut, cell::RefCell};

const BTREE_DEGREE: usize = 4;
//
// #[derive(Debug)]
// struct BTree {
//     root: Rc<RefCell<BNode>>,
// }
//
// impl BTree {
//     fn new() -> Self {
//         Self {
//             root: Rc::new(RefCell::new(BNode::new())),
//         }
//     }
//     fn insert(&mut self, key: i32) {
//         self.root.borrow_mut().get_mut().insert(key);
//         loop {
//             match &self.root.borrow().parent {
//                 None => break,
//                 Some(parent) => {
//                     self.root = Rc::clone(&parent);
//                 }
//             }
//         }
//     }
//     fn display(&self) {
//         BTree::displ(self.root);
//     }
//     fn displ(node: Rc<RefCell<BNode>>) {
//         print!("key: ");
//         for k in node.borrow().key {
//             print!("{} ", k);
//         }
//         println!();
//         for child in node.borrow().children {
//             print!(">> ");
//             BTree::displ(Rc::new(RefCell::new(*child.clone())));
//         }
//     }
// }

#[derive(Debug, Clone)]
struct BNode {
    parent: Option<Rc<RefCell<BNode>>>,
    children: Vec<Box<BNode>>,
    key: Vec<i32>,
}

impl BNode {
    fn new() -> Self {
        Self {
            parent: None,
            children: Vec::new(),
            key: Vec::new(),
        }
    }
    fn insert(&mut self, key: i32) {
        if self.children.len() == 0 {
            self.key.push(key);
            self.key.sort();
            if self.key.len() == BTREE_DEGREE {
                self.split();
            }
            return;
        }
        let mut child: usize = 0;
        for i in 0..self.key.len() {
            if self.key[i] > key {
                break;
            }
            child += 1;
        }
        self.children[child].insert(key);
    }
    fn split(&mut self) {
        let mut split = Box::new(BNode::new());
        let mid = BTREE_DEGREE / 2;
        for _ in (mid + 1)..self.key.len() {
            split.key.push(self.key.pop().unwrap());
            if self.children.len() > 0 {
                split.children.push(self.children.pop().unwrap());
            }
        }
        split.key.reverse();
        if self.children.len() > 0 {
            split.children.push(self.children.pop().unwrap());
            split.children.reverse();
        }
        let to_root = self.key.pop().unwrap();
        match self.parent.borrow_mut() {
            Some(parent) => {
                split.parent = Some(Rc::clone(&parent));
                parent
                    .borrow_mut()
                    .get_mut()
                    .force_insert(to_root, Box::new(self.clone()), split);
            }
            None => {
                let mut parent = Rc::new(RefCell::new(BNode::new()));
                split.parent = Some(Rc::clone(&parent));
                parent
                    .borrow_mut()
                    .get_mut()
                    .force_insert(to_root, Box::new(self.clone()), split);
                self.parent = Some(parent);
            }
        }
    }
    fn force_insert(&mut self, key: i32, left: Box<BNode>, right: Box<BNode>) {
        if self.key.len() == 0 {
            self.key.push(key);
            self.children.push(left);
            self.children.push(right);
            return;
        }
        let mut insert_pos: usize = 0;
        for i in 0..self.key.len() {
            if self.key[i] > key {
                break;
            }
            insert_pos = i;
        }
        self.key.push(key);
        self.children.push(right);
        for i in (insert_pos + 1..self.key.len()).rev() {
            self.key.swap(i, i - 1);
            self.children.swap(i, i - 1);
        }
        if self.key.len() == BTREE_DEGREE {
            self.split();
        }
    }
}

fn main() {
    // let mut btree = BTree::new();
    // btree.insert(10);
    // btree.insert(20);
    // btree.insert(30);
    // btree.insert(40);
    // btree.insert(21);
    // btree.insert(25);
    // btree.display();
}
