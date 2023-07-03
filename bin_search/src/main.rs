fn main() {
    let test = vec![1, 4, 5, 99, 567, 3123, 7956, 11036];
    let find = 4;

    if let Some(res) = binary_search(&test, find) {
        assert_eq!(res, 1);
        println!("The index of '4' is: {}", res);
    }
}

fn binary_search<T: Ord + Eq + Copy>(vec: &Vec<T>, item: T) -> Option<usize> {
    let mut low = 0;
    let mut high = vec.len() - 1;

    while low <= high {
        let mid = (low + high) / 2;
        let guess = vec[mid];
        if guess == item {
            return Some(mid);
        }
        if guess > item {
            high = mid - 1;
        } else {
            low = mid + 1;
        }
    }
    None
}
