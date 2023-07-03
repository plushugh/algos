use std::collections::LinkedList;

fn main() {
    let mut cache = Cache::new();

    let some_num = cache.get_or_create(String::from("some_num"));

    println!("Got value: {}", some_num);

    let some_of_the_same_num = cache.get_or_create(String::from("some_num"));

    println!("Got value: {}", some_of_the_same_num);

    let some_other_num = cache.get_or_create(String::from("some_other_num"));

    println!("Got value: {}", some_other_num);

    let some_of_the_same_other_num = cache.get_or_create(String::from("some_other_num"));

    println!("Got value: {}", some_of_the_same_other_num);
}

struct Cache {
    cache: HashMap
}

impl Cache {
    fn new() -> Self {
        println!("Creating cache");

        Cache {
            cache: HashMap::new()
        }
    }

    fn get_or_create(&mut self, key: String) -> usize {
        match self.cache.get(&key) {
            Some(value) => {
                println!("Cache hit!");

                *value
            },
            None => {
                println!("Cache miss! generating value...");

                let value = rand::random::<usize>();
                self.cache.insert(key, value);
                value
            }
        }
    }
}




struct HashMap {
    buckets: Vec<LinkedList<(String, usize)>>,
}

impl HashMap {
    fn new() -> Self {
        HashMap {
            buckets: vec![LinkedList::new(); 256],
        }
    }

    fn insert(&mut self, key: String, value: usize) {
        let bytes = key.as_bytes().iter();
        let sum = bytes.fold(0, |acc, &b| acc + b as usize);
        let hash: u8 = (sum % 256).try_into().unwrap();

        let bucket = &mut self.buckets[hash as usize];
        
        if let Some(node) = bucket.iter_mut().find(|(k, _)| *k == key) {
            node.1 = value;
        } else {
            bucket.push_back((key.clone(), value));
        }
    }

    fn get(&self, key: &String) -> Option<&usize> {
        let bytes = key.as_bytes().iter();
        let sum = bytes.fold(0, |acc, &b| acc + b as usize);
        let hash: u8 = (sum % 256).try_into().unwrap();

        let bucket = &self.buckets[hash as usize];
        bucket.iter().find(|(k, _)| *k == *key).map(|(_, v)| v)
    }

    // fn remove(&mut self, key: String) {
    //     let bytes = key.as_bytes().iter();
    //     let sum = bytes.fold(0, |acc, &b| acc + b as usize);
    //     let hash: u8 = (sum % 256).try_into().unwrap();

    //     let bucket = &mut self.buckets[hash as usize];
    //     if let Some(node) = bucket.iter_mut().find(|(k, _)| *k == key) {
    //         bucket.remove(node); // unstable library feature
    //     }
    // }
}