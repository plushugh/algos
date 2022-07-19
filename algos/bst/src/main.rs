use std::cmp::Ordering;

fn main() {
    let mut bst = BST::new();
    bst.insert("a", "2");
    bst.insert("b", "1");
    bst.insert("c", "3");
    println!("{}", bst.get(&"b").unwrap());
}

struct Node<K, V> {
    left: Option<Box<Node<K, V>>>,
    right: Option<Box<Node<K, V>>>,
    key: K,
    value: V,
}

impl<K: Ord, V> Node<K, V> {
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
            Ordering::Less => match self.left {
                None => self.left = Some(Box::new(n)),
                Some(ref mut l) => l.insert(n),
            },
            Ordering::Greater => match self.right {
                None => self.right = Some(Box::new(n)),
                Some(ref mut r) => r.insert(n),
            },
            _ => {}
        }
    }

    fn get(&self, key: &K) -> Option<&V> {
        match key.cmp(&self.key) {
            Ordering::Equal => Some(&self.value),
            Ordering::Less => {
                if let Some(ref l) = self.left {
                    l.get(key)
                } else {
                    None
                }
            }
            Ordering::Greater => {
                if let Some(ref r) = self.right {
                    r.get(key)
                } else {
                    None
                }
            }
        }
    }
}

struct BST<K, V> {
    root: Option<Box<Node<K, V>>>,
}
impl<K: Ord, V> BST<K, V> {
    fn new() -> BST<K, V> {
        BST { root: None }
    }
    fn insert(&mut self, key: K, value: V) {
        if let Some(ref mut r) = self.root {
            r.insert(Node::new(key, value));
        } else {
            self.root = Some(Box::new(Node::new(key, value)));
        }
    }
    fn get(&self, k: &K) -> Option<&V> {
        match self.root {
            None => None,
            Some(ref r) => r.get(k),
        }
    }
}
