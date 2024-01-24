fn main() {
    let mut test = vec![11036, 3123, 5, 99, 1, 4, 7956, 567];

    quicksort_3way(&mut test, 0, test.len() - 1);

    assert_eq!(test, vec![1, 4, 5, 99, 567, 3123, 7956, 11036]);
}

fn quicksort_3way<T: Ord + Copy>(arr: &mut [T], lo: usize, hi: usize) {
    if hi <= lo {
        return;
    }

    let mut lt = lo;
    let mut gt = hi;
    let mut i = lo + 1;
    let pivot = arr[lo];

    while i <= gt {
        if arr[i] < pivot {
            arr.swap(lt, i);
            lt += 1;
            i += 1;
        } else if arr[i] > pivot {
            arr.swap(i, gt);
            gt -= 1;
        } else {
            i += 1;
        }
    }

    quicksort_3way(arr, lo, lt - 1);
    quicksort_3way(arr, gt + 1, hi);
}
