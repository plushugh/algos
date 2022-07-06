use std::collections::HashMap;

fn main() {
    let mut cache = Cache::new();

    let some_num = cache.get_or_create(String::from("some_num"));

    println!("Got value: {}", some_num);

    let some_of_the_same_num = cache.get_or_create(String::from("some_num"));

    println!("Got value: {}", some_of_the_same_num);
}

struct Cache {
    cache: HashMap<String, usize>
}

impl Cache {
    fn new() -> Cache {
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