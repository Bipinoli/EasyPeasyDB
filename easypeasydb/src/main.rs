#[allow(dead_code)]

const BTREE_DEGREE: usize = 4;

#[derive(Debug, Clone)]
struct BTree {
    parent: Option<Box<BTree>>,
    children: Vec<Box<BTree>>,
    key: Vec<i32>,
}
impl BTree {
    fn new() -> Self {
        Self {
            parent: None,
            children: Vec::new(),
            key: Vec::new(),
        }
    }
    fn insert(&mut self, key: i32) {
        self.key.push(key);
        self.key.sort();
        if self.key.len() == BTREE_DEGREE {
            self.split();
        }
    }
    fn split(&mut self) {
        todo!("handle children properly");
        let mut split = Box::new(BTree::new());
        for _ in 0..(BTREE_DEGREE / 2) {
            split.insert(self.key.pop().unwrap());
        }
        let to_root = self.key.pop().unwrap();
        match &mut self.parent {
            None => {
                let mut parent = Box::new(BTree::new());
                parent.key.push(to_root);
                parent.children.push(Box::new(self.clone()));
                parent.children.push(Box::new(*split));
            }
            Some(parent) => {
                parent.force_insert(to_root, split);
            }
        }
    }
    fn force_insert(&mut self, key: i32, child: Box<BTree>) {
        let mut insert_pos: usize = 0;
        for i in 0..self.key.len() {
            if self.key[i] > key {
                break;
            }
            insert_pos = i;
        }
        self.key.push(key);
        self.children.push(child);
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
    let mut btree = BTree::new();
    for i in 1..10 {
        btree.insert(i);
    }
    dbg!(&btree);
}
