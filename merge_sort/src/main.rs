fn main() {
    let mut test = vec![11036,3123,5,99,1,4,7956,567];

    merge_sort(&mut test);

    assert_eq!(test, vec![1,4,5,99,567,3123,7956,11036]);
}

fn merge_sort<T: Ord + Copy>(arr: &mut [T]) {
    if arr.len() > 1 {
        let mid = arr.len() / 2;
        let mut left = arr[..mid].to_vec();
        let mut right = arr[mid..].to_vec();

        merge_sort(&mut left);
        merge_sort(&mut right);

        merge(&mut left, &mut right, arr);
    }
} 

fn merge<T: Ord + Copy>(left: &mut Vec<T>, right: &mut Vec<T>, arr: &mut [T]) {
    let mut i = 0;
    let mut j = 0;
    let mut k = 0;

    while i < left.len() && j < right.len() {
        if left[i] <= right[j] {
            arr[k] = left[i];
            i += 1;
        } else {
            arr[k] = right[j];
            j += 1;
        }
        k += 1;
    }

    while i < left.len() {
        arr[k] = left[i];
        i += 1;
        k += 1;
    }

    while j < right.len() {
        arr[k] = right[j];
        j += 1;
        k += 1;
    }
}