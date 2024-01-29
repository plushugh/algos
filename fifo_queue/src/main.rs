use std::collections::LinkedList;

pub struct Queue<T> {
    list: LinkedList<T>,
}

impl<T> Queue<T> {
    pub fn new() -> Self {
        Queue {
            list: LinkedList::new(),
        }
    }

    pub fn push(&mut self, value: T) {
        self.list.push_back(value);
    }

    pub fn pop(&mut self) -> Option<T> {
        self.list.pop_front()
    }
}

fn main() {
    let mut queue = Queue::new();
    queue.push(1);
    queue.push(2);
    println!("{:?}", queue.pop());
    queue.push(3);
    println!("{:?}", queue.pop());
    println!("{:?}", queue.pop());
}
