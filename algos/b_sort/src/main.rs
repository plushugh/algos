fn main() {
    let mut test = vec![11036, 3123, 5, 99, 1, 4, 7956, 567];

    bubble_sort(&mut test);

    assert_eq!(test, vec![11036, 7956, 3123, 567, 99, 5, 4, 1]);
}

fn bubble_sort(vec: &mut Vec<usize>) {
    let mut i: usize = 0;
    let mut j: usize = 0;
    let mut temp: usize;
    while i < vec.len() {
        while j < vec.len() {
            if vec[i] > vec[j] {
                temp = vec[i];
                vec[i] = vec[j];
                vec[j] = temp;
            }
            j += 1;
        }
        i += 1;
        j = 0;
    }
}
