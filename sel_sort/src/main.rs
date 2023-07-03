fn main() {
    let mut test = vec![11036,3123,5,99,1,4,7956,567];

    selection_sort(&mut test);

    assert_eq!(test, vec![1,4,5,99,567,3123,7956,11036]);
}

fn selection_sort<T: Ord + Copy>(arr: &mut [T]) {
    for i in 0..arr.len() {
        let mut min = i;
        for j in i..arr.len() {
            if arr[j] < arr[min] {
                min = j;
            }
        }
        arr.swap(i, min);
    }
}
