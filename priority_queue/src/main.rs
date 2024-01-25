// priotiry_queue and heapsort

use std::cmp::Ordering;
use std::fmt::Debug;

#[derive(Debug)]
struct PriorityQueue<T: Ord + Debug> {
    heap: Vec<T>,
}

impl<T: Ord + Debug> PriorityQueue<T> {
    fn new() -> Self {
        PriorityQueue { heap: Vec::new() }
    }

    fn insert(&mut self, val: T) {
        self.heap.push(val);
        self.swim(self.heap.len() - 1);
    }

    fn del_max(&mut self) -> Option<T> {
        if self.heap.is_empty() {
            return None;
        }

        let max = self.heap.swap_remove(0);
        self.sink(0);

        Some(max)
    }

    fn swim(&mut self, mut k: usize) {
        while k > 0 && self.less(k / 2, k) {
            self.heap.swap(k / 2, k);
            k = k / 2;
        }
    }

    fn sink(&mut self, mut k: usize) {
        while 2 * k < self.heap.len() {
            let mut j = 2 * k;

            if j < self.heap.len() - 1 && self.less(j, j + 1) {
                j += 1;
            }

            if !self.less(k, j) {
                break;
            }

            self.heap.swap(k, j);
            k = j;
        }
    }

    fn less(&self, i: usize, j: usize) -> bool {
        match self.heap[i].cmp(&self.heap[j]) {
            Ordering::Less => true,
            _ => false,
        }
    }
}

fn heapsort<T: Ord + Copy + Debug>(arr: &mut [T]) {
    let mut pq = PriorityQueue::new();

    for i in 0..arr.len() {
        pq.insert(arr[i]);
    }

    for i in 0..arr.len() {
        arr[i] = pq.del_max().unwrap();
    }
}

fn main() {
    let mut test = vec![11036, 3123, 5, 99, 1, 4, 7956, 567];

    heapsort(&mut test);

    assert_eq!(test, vec![11036, 7956, 3123, 567, 99, 5, 4, 1]);
}
