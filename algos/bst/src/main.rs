use std::{cell::RefCell, cmp::Ordering, rc::Rc};

fn main() {
    let mut bst = BST::new();
    bst.insert("a", "2");
    bst.insert("b", "1");
    bst.insert("c", "3");
    println!("{}", bst.get(&"b").unwrap());
}

struct Node<K, V> {
    left: Option<Rc<RefCell<Node<K, V>>>>,
    right: Option<Rc<RefCell<Node<K, V>>>>,
    key: K,
    value: V,
}

impl<K: Ord, V: Copy> Node<K, V> {
    fn new(key: K, value: V) -> Node<K, V> {
        Node {
            left: None,
            right: None,
            key,
            value,
        }
    }

    fn insert(&mut self, n: Node<K, V>) {
        match n.key.cmp(&self.key) {
            Ordering::Less => match &self.left {
                None => self.left = Some(Rc::new(RefCell::new(n))),
                Some(l) => l.borrow_mut().insert(n),
            },
            Ordering::Greater => match &self.right {
                None => self.right = Some(Rc::new(RefCell::new(n))),
                Some(r) => r.borrow_mut().insert(n),
            },
            _ => {}
        }
    }

    fn get(&self, key: &K) -> Option<V> {
        match key.cmp(&self.key) {
            Ordering::Equal => Some(self.value),
            Ordering::Less => {
                if let Some(l) = &self.left {
                    l.borrow().get(key)
                } else {
                    None
                }
            }
            Ordering::Greater => {
                if let Some(r) = &self.right {
                    r.borrow().get(key)
                } else {
                    None
                }
            }
        }
    }
}

struct BST<K, V> {
    root: Option<Rc<RefCell<Node<K, V>>>>,
}
impl<K: Ord, V: Copy> BST<K, V> {
    fn new() -> BST<K, V> {
        BST { root: None }
    }
    fn insert(&mut self, key: K, value: V) {
        if let Some(r) = &self.root {
            r.borrow_mut().insert(Node::new(key, value));
        } else {
            self.root = Some(Rc::new(RefCell::new(Node::new(key, value))));
        }
    }
    fn get(&self, k: &K) -> Option<V> {
        match &self.root {
            None => None,
            Some(r) => r.borrow().get(k),
        }
    }
}
