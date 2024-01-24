fn main() {
    let mut test = vec![11036, 3123, 5, 99, 1, 4, 7956, 567];

    quick_sort(&mut test);

    assert_eq!(test, vec![1, 4, 5, 99, 567, 3123, 7956, 11036]);
}

fn quicksort_partition<T: Ord + Copy>(arr: &mut [T], lo: usize, hi: usize) -> usize {
    let pivot = arr[lo];
    let mut i = lo + 1;
    let mut j = lo + 1;

    while j <= hi {
        if arr[j] < pivot {
            arr.swap(i, j);
            i += 1;
        }
        j += 1;
    }

    arr.swap(lo, i - 1);
    i - 1
}
